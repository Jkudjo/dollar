use rand::Rng;
use num_format::{Locale, ToFormattedString};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use crossterm::{
    ExecutableCommand,
    terminal::{Clear, ClearType},
    cursor,
    style::{Color, SetForegroundColor, ResetColor},
};
use std::io::Write as _;
use indicatif::{ProgressBar, ProgressStyle};

const FILE_PATH: &str = "total_amount.txt";
const INTERVAL_SECS: u64 = 5; // Interval between updates in seconds

fn main() -> io::Result<()> {
    // Initialize the progress bar
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner()
        .template("{spinner:.green} {msg}")
        .tick_strings(&["⠋", "⠙", "⠚", "⠉", "⠈"]));
    pb.set_message("Siphoning funds...");

    // Initialize terminal styling
    let mut stdout = std::io::stdout();
    stdout.execute(Clear(ClearType::All))?; // Clear the terminal
    stdout.execute(cursor::Hide)?; // Hide the cursor

    loop {
        let new_amount = generate_random_amount();
        let mut total_amount = read_total_amount()?;
        total_amount += new_amount;
        write_total_amount(total_amount)?;

        let new_amount_formatted = new_amount.to_formatted_string(&Locale::en);
        let total_amount_formatted = total_amount.to_formatted_string(&Locale::en);

        // Clear previous output lines
        stdout.execute(Clear(ClearType::CurrentLine))?;
        stdout.execute(cursor::MoveToColumn(0))?;
        stdout.execute(Clear(ClearType::CurrentLine))?;
        stdout.execute(cursor::MoveToColumn(0))?;

        // Print styled output
        print!("{}Your bot just siphoned ${} into your account\n", 
            SetForegroundColor(Color::Cyan), new_amount_formatted);
        print!("{}Total amount siphoned: ${}\n", 
            SetForegroundColor(Color::Green), total_amount_formatted);
        print!("{}", ResetColor); // Reset color

        // Update progress bar
        pb.tick();
        sleep(Duration::new(INTERVAL_SECS, 0));
    }
}

fn generate_random_amount() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1_000..100_000) // Random amount between $1,000 and $100,000
}

fn read_total_amount() -> io::Result<u32> {
    if !Path::new(FILE_PATH).exists() {
        return Ok(0);
    }

    let mut file = File::open(FILE_PATH)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.trim().parse().unwrap_or(0))
}

fn write_total_amount(total_amount: u32) -> io::Result<()> {
    let mut file = OpenOptions::new().write(true).create(true).open(FILE_PATH)?;
    writeln!(file, "{}", total_amount)?;
    Ok(())
}

