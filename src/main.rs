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

    let mut total_amount = read_total_amount()?;
    loop {
        // Display current status
        let total_amount_formatted = total_amount.to_formatted_string(&Locale::en);
        stdout.execute(Clear(ClearType::CurrentLine))?;
        stdout.execute(cursor::MoveToColumn(0))?;
        println!("\nCurrent total: \x1b[32m${}\x1b[0m", total_amount_formatted); // Green text

        // Clear and display menu options
        println!("\nChoose your action:");
        println!("1. 💰 Siphon small amount");
        println!("2. ⚠️ Siphon large amount (risk of getting caught)");
        println!("3. ❌ Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        let choice: u32 = choice.trim().parse().unwrap_or(0);

        let new_amount = match choice {
            1 => generate_random_amount(1_000, 10_000),
            2 => {
                let risk: f64 = rand::thread_rng().gen(); // Generate a random risk factor
                if risk < 0.2 { // 20% chance of getting caught
                    println!("\x1b[31mYou got caught! Game over.\x1b[0m"); // Red text for error
                    break;
                }
                generate_random_amount(10_000, 100_000)
            },
            3 => {
                println!("\x1b[34mExiting the game. Final total: ${}\x1b[0m", total_amount_formatted); // Blue text
                break;
            },
            _ => {
                println!("\x1b[31mInvalid choice, try again.\x1b[0m"); // Red text for error
                continue;
            }
        };

        // Update total amount and write to file
        total_amount += new_amount;
        write_total_amount(total_amount)?;

        let new_amount_formatted = new_amount.to_formatted_string(&Locale::en);
        println!("You siphoned \x1b[32m${}\x1b[0m. New total: \x1b[32m${}\x1b[0m", new_amount_formatted, total_amount_formatted);

        // Check for random event
        let event_chance: f64 = rand::thread_rng().gen();
        if event_chance < 0.1 {
            let penalty = generate_random_amount(10_000, 50_000);
            total_amount -= penalty;
            println!("\x1b[31mAudit alert! Penalty: ${}. New total: ${}\x1b[0m", penalty, total_amount); // Red text for penalty
        }

        // Update progress bar
        pb.tick();
        sleep(Duration::new(INTERVAL_SECS, 0));

        // Check win condition
        if total_amount >= GOAL_AMOUNT {
            println!("\x1b[34mCongratulations! You reached ${}. You win!\x1b[0m", GOAL_AMOUNT); // Blue text for win
            break;
        }
    }

    // Restore cursor visibility before exiting
    stdout.execute(cursor::Show)?;
    Ok(())
}