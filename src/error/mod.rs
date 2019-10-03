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

impl From<crossbeam_channel::RecvError> for AppError {
    fn from(src: crossbeam_channel::RecvError) -> AppError {
        AppError::Critical {
            message: format!("Failed to receive message on a channel!\n{:?}", src),
        }
    }
}

impl From<crossbeam_channel::SendError<crate::runtime::Message>> for AppError {
    fn from(src: crossbeam_channel::SendError<crate::runtime::Message>) -> AppError {
        AppError::Critical {
            message: format!("Failed to send message on a channel!\n{:?}", src),
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
