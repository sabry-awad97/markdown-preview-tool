use std::{
    fs, io,
    path::Path,
    process::{exit, Command, Stdio},
    thread,
    time::{Duration, Instant},
};

use serde::Serialize;
use structopt::StructOpt;

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

fn main() {
    let opt = Opt::from_args();
    if let Err(err) = run(opt) {
        eprintln!("Error: {}", err);
        exit(1);
    }
}

fn run(opt: Opt) -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string(&opt.file)?;
    let html_data = parse_content(&input)?;

    let temp_dir = tempfile::Builder::new().prefix("mdp").tempdir()?;
    let temp_file_path = temp_dir.path().join("temp.html");

    save_html(&temp_file_path, &html_data)?;

    Ok(preview(&temp_file_path)?)
}

fn parse_content(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut unsafe_html = String::new();
    pulldown_cmark::html::push_html(&mut unsafe_html, pulldown_cmark::Parser::new(input));
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

fn save_html<P: AsRef<Path>>(filename: P, html_data: &str) -> io::Result<()> {
    fs::write(filename, html_data)?;
    Ok(())
}

fn preview<P: AsRef<Path>>(filename: P) -> io::Result<()> {
    let os: &str = std::env::consts::OS;

    let prog = match os {
        "windows" => "cmd.exe",
        _ => return Err(io::Error::new(io::ErrorKind::Other, "OS not supported")),
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
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Failed to open file in browser",
                    ));
                }
                break;
            }
            Ok(None) => {
                if start_time.elapsed() > Duration::from_secs(10) {
                    child.kill()?;
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Timed out waiting for browser to open file",
                    ));
                }
                thread::sleep(Duration::from_millis(1000));
            }
            Err(e) => {
                child.kill()?;
                return Err(e);
            }
        }
    }

    Ok(())
}
