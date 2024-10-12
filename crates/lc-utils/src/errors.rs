use std::fmt::Display;

use anyhow::anyhow;

pub struct AppError(anyhow::Error);

impl From<&str> for AppError {
    fn from(value: &str) -> Self {
        AppError(anyhow!("{}", value))
    }
}

impl From<anyhow::Error> for AppError {
    fn from(value: anyhow::Error) -> Self {
        AppError(value)
    }
}

impl Into<anyhow::Error> for AppError {
    fn into(self) -> anyhow::Error {
        self.0
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
