use std::fmt::Display;

use anyhow::anyhow;

pub struct AppError(Option<anyhow::Error>);

impl From<Option<&str>> for AppError {
    fn from(value: Option<&str>) -> Self {
        if let Some(value) = value {
            AppError(Some(anyhow!("{}", value)))
        } else {
            AppError(None)
        }
    }
}

impl From<&str> for AppError {
    fn from(value: &str) -> Self {
        AppError(Some(anyhow!("{}", value)))
    }
}

impl From<anyhow::Error> for AppError {
    fn from(value: anyhow::Error) -> Self {
        AppError(Some(value))
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(value) = &self.0 {
            write!(f, "{}", value)
        } else {
            write!(f, "")
        }
    }
}
