use colored::Colorize;



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