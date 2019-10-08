use std::error;
use std::fmt;

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    Critical {
        message: String,
    },
    IOError {
        message: String,
        source: std::io::Error,
    },
}

impl error::Error for AppError {}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Critical { message } => write!(f, "Critical Failure: {}", message),
            AppError::IOError { message, source } => write!(f, "IO Error: {}\n{}", message, source),
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(src: std::io::Error) -> AppError {
        AppError::IOError {
            message: format!("An IO error has occurred!\n{:?}", src),
            source: src,
        }
    }
}

impl From<serde_yaml::Error> for AppError {
    fn from(src: serde_yaml::Error) -> AppError {
        AppError::Critical {
            message: format!("Could not parse YAML!\n{:?}", src),
        }
    }
}

impl From<templar::error::TemplarError> for AppError {
    fn from(src: templar::error::TemplarError) -> AppError {
        AppError::Critical {
            message: format!("{}", src),
        }
    }
}

impl From<regex::Error> for AppError {
    fn from(src: regex::Error) -> AppError {
        AppError::Critical {
            message: format!("{}", src),
        }
    }
}
