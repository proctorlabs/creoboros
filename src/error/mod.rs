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

impl From<std::io::Error> for AppError {
    fn from(src: std::io::Error) -> AppError {
        AppError::IOError {
            message: format!("An IO error has occurred!\n{:?}", src),
            source: src,
        }
    }
}
