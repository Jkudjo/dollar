use std::io;
use std::{fs::{OpenOptions}, io::{Read, Write}};
use indicatif::{ProgressBar, ProgressStyle};
use crossterm::{
    ExecutableCommand,
    terminal::{Clear, ClearType},
    cursor,
    Result,
};
use num_format::{ToFormattedString, Locale};
use rand::Rng;


const GOAL_AMOUNT: u64 = 1_000_000;
const SAVE_FILE: &str = "total_amount.txt";

// ... (read_total_amount and write_total_amount functions remain the same)

fn generate_random_amount(min: u64, max: u64) -> u64 {
    rand::thread_rng().gen_range(min..=max)
}


fn main() -> Result<()> {
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner()
        .template("{spinner:.green} {msg}")
        .tick_strings(&["⠋", "⠙", "⠚", "⠉", "⠈"]));
    pb.set_message("Siphoning funds...");

    let mut stdout = io::stdout();
    stdout.execute(Clear(ClearType::All))?;
    stdout.execute(cursor::Hide)?;

    let mut total_amount = read_total_amount()?;

    loop {
        let total_amount_formatted = total_amount.to_formatted_string(&Locale::en);
        println!("Current total: ${}", total_amount_formatted);

        // ... (Game logic to get player input for siphoning)

        let new_amount = generate_random_amount(500, 10_000); //Example
        total_amount += new_amount;
        
         // Example game logic - replace with your actual logic
        if rand::thread_rng().gen_bool(0.1) {
             let penalty = generate_random_amount(100, 5000);
             total_amount -= penalty; // Apply random penalty
             println!("Penalty of ${} applied!", penalty.to_formatted_string(&Locale::en));
         }


        pb.tick();

        if total_amount >= GOAL_AMOUNT {
            println!("You won!");
            write_total_amount(total_amount)?;
            break;
        }


        write_total_amount(total_amount)?;


    }
    stdout.execute(cursor::Show)?;
    Ok(())
}

