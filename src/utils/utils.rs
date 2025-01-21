use bevy::prelude::{Query, With};
use bevy::window::{PrimaryWindow, Window};
use colored::{Color, Colorize};
use reqwest::blocking::get;
use std::io::{copy, Read};
use std::fs::File;
use std::io::{self, stdin, Write};
// use std::process::Command;
use std::{thread, time::Duration};

use super::logger::*;

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

pub fn read_input(prompt: &str, color: Color) -> Option<String> {
    print!("{}", prompt.color(color).bold());
    io::stdout().flush().unwrap();
    let mut input = String::new();
    if stdin().read_line(&mut input).is_err() {
        display_error("Failed to read input");
        return None;
    }

    Some(input.trim().to_string())
}

fn print_slowly(text: &str) {
    for ch in text.chars() {
        print!("{}", ch);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(60));
    }
    println!();
}

pub fn print_menu() {
    const BORDER: &str = "════════════════════════════════════════";
    let title = "GATSA-GATSA";
    print_slowly(&format!(
        "{}",
        "          WELCOME TO THE GAME MENU".blue().bold()
    ));

    let options = vec!["1 - Start as Server", "2 - Start as Client"];

    println!("╔{}╗", BORDER.blue().bold());
    println!("║{:^40}║", title.blue().bold());
    println!("╠{}╣", BORDER.blue().bold());

    for option in &options {
        println!("║{:^40}║", option);
    }

    println!("╚{}╝", BORDER.blue().bold());
}

pub fn get_user_choice() -> Option<u32> {
    print_menu();
    let choice = read_input("          Enter your choice: ", Color::Blue)?;
    match choice.parse::<u32>() {
        Ok(num) => Some(num),
        Err(_) => {
            display_error("Invalid choice. Please enter a valid number.");
            None
        }
    }
}

// ========================================== CLIENT ===========================================
pub fn get_user_input() -> Option<(String, String)> {
    clear_screen();

    println!(
        "{}",
        "                                  ╔════════════════════════════════╗"
            .green()
            .bold()
    );
    println!(
        "{}",
        "                                  ║  WELCOME TO OUR PAYT-GAME FPS  ║"
            .green()
            .bold()
    );
    println!(
        "{}",
        "                                  ╚════════════════════════════════╝"
            .green()
            .bold()
    );

    let ip = read_input("Enter The Server IP Address: ", Color::Yellow)?;
    let name = read_input("Enter Your Name: ", Color::Yellow)?;

    println!(
        "{}",
        "                                     ============================"
            .green()
            .bold()
    );

    Some((name, ip))
}

pub fn get_window_dimensions(windows: &Query<&Window, With<PrimaryWindow>>) -> (f32, f32) {
    let window = windows.single();
    (window.width(), window.height())
}

// pub fn get_models(url: &str, output_path: &str) -> io::Result<()> {
//     let response = get(url).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
//     let mut file = File::create(output_path)?;
//     let mut content = response;
//     let mut content_ref = content.take(content.content_length().unwrap_or(0));
//     copy(
//         &mut content_ref,
//         &mut file,
//     )?;
//     Ok(())
// }
