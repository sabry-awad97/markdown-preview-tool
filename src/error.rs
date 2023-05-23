use std::fmt;
use std::io;

#[derive(Debug)]
pub enum MarkdownPreviewError {
    IoError(io::Error),
    TemplateRenderingError(tera::Error),
}

impl From<io::Error> for MarkdownPreviewError {
    fn from(error: io::Error) -> Self {
        MarkdownPreviewError::IoError(error)
    }
}

impl From<tera::Error> for MarkdownPreviewError {
    fn from(error: tera::Error) -> Self {
        MarkdownPreviewError::TemplateRenderingError(error)
    }
}

impl std::error::Error for MarkdownPreviewError {}

impl fmt::Display for MarkdownPreviewError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MarkdownPreviewError::IoError(ref inner) => write!(f, "IO Error: {}", inner),
            MarkdownPreviewError::TemplateRenderingError(ref inner) => {
                write!(f, "Template Rendering Error: {}", inner)
            }
        }
    }
}
