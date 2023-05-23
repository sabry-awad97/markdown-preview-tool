use std::{
    fs, io,
    path::Path,
    process::{exit, Command, Stdio},
    thread,
    time::{Duration, Instant},
};

use error::MarkdownPreviewError;
use serde::Serialize;
use structopt::StructOpt;

mod error;

const DEFAULT_TEMPLATE: &str = r#"
    <!DOCTYPE html>
    <html>
        <head>
            <meta http-equiv="content-type" content="text/html; charset=utf-8">
            <title>
                {{ title }}
            </title>
        </head>
        <body>
            {{ body }}
        </body>
    </html>
"#;

#[derive(Debug, Serialize)]
struct Content {
    title: String,
    body: String,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Markdown Preview Tool",
    about = "Converts Markdown to HTML and opens it in a web browser for preview."
)]
struct Opt {
    #[structopt(short, long, help = "Markdown file to preview")]
    file: String,
}

struct MarkdownPreviewTool {
    opt: Opt,
}

impl MarkdownPreviewTool {
    fn new(opt: Opt) -> Self {
        Self { opt }
    }

    fn run(&self) -> Result<(), MarkdownPreviewError> {
        let input = fs::read_to_string(&self.opt.file)?;
        let html_data = self.parse_content(&input)?;

        let temp_dir = tempfile::Builder::new().prefix("mdp").tempdir()?;
        let temp_file_path = temp_dir.path().join("temp.html");

        self.save_html(&temp_file_path, &html_data)?;

        self.preview(&temp_file_path)?;

        Ok(())
    }

    fn parse_content(&self, input: &str) -> Result<String, MarkdownPreviewError> {
        let parser = pulldown_cmark::Parser::new_ext(input, pulldown_cmark::Options::all());
        let mut unsafe_html = String::new();
        pulldown_cmark::html::push_html(&mut unsafe_html, parser);
        let sanitized_html = ammonia::Builder::new().clean(&unsafe_html).to_string();

        let title = "Markdown Preview Tool".to_owned();
        let body = sanitized_html;

        let rendered = tera::Tera::one_off(
            DEFAULT_TEMPLATE,
            &tera::Context::from_serialize(Content { title, body })?,
            false,
        )?;

        Ok(rendered)
    }

    fn save_html<P: AsRef<Path>>(
        &self,
        filename: P,
        html_data: &str,
    ) -> Result<(), MarkdownPreviewError> {
        fs::write(filename, html_data)?;
        Ok(())
    }

    fn preview<P: AsRef<Path>>(&self, filename: P) -> Result<(), MarkdownPreviewError> {
        let os: &str = std::env::consts::OS;

        let prog = match os {
            "windows" => "cmd.exe",
            _ => {
                return Err(MarkdownPreviewError::IoError(io::Error::new(
                    io::ErrorKind::Other,
                    "OS not supported",
                )))
            }
        };

        let mut cmd = Command::new(prog);
        cmd.args(&["/C", "start", "chrome"])
            .arg(filename.as_ref())
            .stdout(Stdio::null())
            .stderr(Stdio::null());

        let mut child = cmd.spawn()?;
        let start_time = Instant::now();
        loop {
            match child.try_wait() {
                Ok(Some(status)) => {
                    if !status.success() {
                        return Err(MarkdownPreviewError::IoError(io::Error::new(
                            io::ErrorKind::Other,
                            "Failed to open file in browser",
                        )));
                    }
                    break;
                }
                Ok(None) => {
                    if start_time.elapsed() > Duration::from_secs(10) {
                        child.kill()?;
                        return Err(MarkdownPreviewError::IoError(io::Error::new(
                            io::ErrorKind::Other,
                            "Timed out waiting for browser to open file",
                        )));
                    }
                    thread::sleep(Duration::from_millis(1000));
                }
                Err(e) => {
                    child.kill()?;
                    return Err(MarkdownPreviewError::IoError(e));
                }
            }
        }

        Ok(())
    }
}

fn main() {
    let opt = Opt::from_args();
    let tool = MarkdownPreviewTool::new(opt);
    if let Err(err) = tool.run() {
        eprintln!("Error: {}", err);
        exit(1);
    }
}
