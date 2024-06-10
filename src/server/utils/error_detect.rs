use std::fmt;
use std::io;
use std::num;
use thiserror::Error;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Error, Debug)]
pub enum AppError {
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),
    
    #[error("Parse error: {0}")]
    ParseError(#[from] num::ParseIntError),
    
    #[error("Configuration error: {0}")]
    ConfigError(#[from] config::ConfigError),
    
    #[error("Custom error: {0}")]
    CustomError(String),
    
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Database error: {0}")]
    DatabaseError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

pub fn log_error_to_file(error: &AppError, file: &str) {
    let mut file = File::create(file).unwrap();
    writeln!(file, "{}", error).unwrap();
}

pub fn read_error_from_file(file: &str) -> Result<AppError, io::Error> {
    let mut file = File::open(file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(AppError::CustomError(contents))
}

pub fn handle_error(error: AppError) {
    match error {
        AppError::IoError(e) => eprintln!("I/O error occurred: {}", e),
        AppError::ParseError(e) => eprintln!("Parse error occurred: {}", e),
        AppError::ConfigError(e) => eprintln!("Configuration error occurred: {}", e),
        AppError::CustomError(e) => eprintln!("Custom error occurred: {}", e),
        AppError::NetworkError(e) => eprintln!("Network error occurred: {}", e),
        AppError::DatabaseError(e) => eprintln!("Database error occurred: {}", e),
    }
}

pub fn convert_to_io_error(error: AppError) -> io::Error {
    match error {
        AppError::IoError(e) => e,
        _ => io::Error::new(io::ErrorKind::Other, error.to_string()),
    }
}
