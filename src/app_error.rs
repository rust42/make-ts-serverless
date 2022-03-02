use std::error::Error;
use std::io::{Error as IOError, ErrorKind };
use std::fmt::{Display, Debug, Formatter, Result};

pub enum AppError {
    MissingName,
    MissingBucketName,
    MissingStackName,
}

impl AppError {

    fn description(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            AppError::MissingName => {
                write!(f, "App name missing")
            },
            AppError::MissingBucketName => {
                write!(f, "Bucket name missing")
            },
            AppError::MissingStackName => {
                write!(f, "Missing stack name")
            }
        }
    }
}

impl Debug for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.description(f)
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.description(f)
    }
}

impl Error for AppError {
    fn description(&self) -> &str {
        "AppError occurred"
    }
}

impl From<AppError> for IOError {
    fn from(error: AppError) -> Self {
        match error {
            AppError::MissingName => IOError::new(ErrorKind::Unsupported, "App name must be provided"),
            AppError::MissingBucketName => IOError::new(ErrorKind::Unsupported, "Bucket name must be given"),
            AppError::MissingStackName => IOError::new(ErrorKind::Unsupported, "Stack name not given"),
        }
    }
}