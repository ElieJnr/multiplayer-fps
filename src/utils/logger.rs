use colored::Colorize;

pub enum ErrorType {
    Serialization,
    Deserialization,
    Network,
    Unknown,
}

pub fn display_error_with_type(error_type: ErrorType, message: &str) {
    let prefix = match error_type {
        ErrorType::Serialization => "[Serialization Error]",
        ErrorType::Deserialization => "[Deserialization Error]",
        ErrorType::Network => "[Network Error]",
        ErrorType::Unknown => "[Unknown Error]",
    };
    display_error(&format!("{} {}", prefix, message));
}

pub fn display_error(message: &str) {
    println!(
        "{} {} {}",
        "❌".red(),
        "[ERROR]".bold().red(),
        message
    );
}

pub fn display_warning(message: &str) {
    println!(
        "{} {} {}",
        "⚠️".yellow(),
        "[WARNING]".bold().yellow(),
        message
    );
}

pub fn display_info(message: &str) {
    println!(
        "{} {} {}",
        "ℹ️".blue(),
        "[INFO]".bold().blue(),
        message
    );
}