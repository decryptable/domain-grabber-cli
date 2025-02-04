mod grabber;
mod ui;

use chalk_rs::Chalk;
use chrono::{Utc, Weekday};
use clap::{Arg, Command};
use debugoff::multi_ptraceme_or_die;
use inquire::{DateSelect, Text};
use obfstr::obfstr;
use std::fs::write;
use std::path::PathBuf;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    multi_ptraceme_or_die();
    let mut chalk = Chalk::new();

    ui::print_banner();

    let matches = Command::new("domain-grabber-cli")
        .version("0.1.0")
        .author("decryptable")
        .arg(
            Arg::new("date")
                .short('d')
                .long("date")
                .value_name("dd.mm.yyyy")
                .help("Date to fetch domains from")
                .required(false),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("PATH")
                .help("Output file path")
                .required(false),
        )
        .get_matches();

    let date = match matches.get_one::<String>("date") {
        Some(d) => d.clone(),
        None => {
            let today = Utc::now().date_naive();
            let selected_date = DateSelect::new(obfstr!("📅 Select a date:"))
                .with_default(today)
                .with_max_date(today)
                .with_week_start(Weekday::Mon)
                .prompt();

            match selected_date {
                Ok(date) => date.format("%d.%m.%Y").to_string(),
                Err(_) => {
                    chalk
                        .red()
                        .bold()
                        .println(&obfstr!("❌ Error: Failed to select a date."));
                    std::process::exit(1);
                }
            }
        }
    };

    let default_output = format!("domains_{}.txt", date);
    let output_path = match matches.get_one::<String>("output") {
        Some(path) => PathBuf::from(path),
        None => {
            let input = Text::new(obfstr!(
                "💾 Enter output file path (or press enter for default):"
            ))
            .with_initial_value(&default_output)
            .prompt();

            match input {
                Ok(path) => PathBuf::from(path),
                Err(_) => {
                    chalk
                        .red()
                        .bold()
                        .println(&obfstr!("❌ Error: Invalid file path."));
                    std::process::exit(1);
                }
            }
        }
    };

    chalk
        .cyan()
        .bold()
        .println(&format!("📅 Fetching domains for date: {}", date));

    let mut sp = ui::show_loading();
    sleep(Duration::from_secs(3)).await;

    match grabber::fetch_domains(&date).await {
        Ok(data) => {
            let domains: Vec<&str> = data.lines().collect();
            let total_domains = domains.len();
            sp.stop_with_message(chalk.green().bold().string(&format!(
                "\n✅ Successfully retrieved {} domains!",
                total_domains
            )));

            if let Err(e) = write(&output_path, data) {
                chalk
                    .red()
                    .bold()
                    .println(&format!("❌ Error saving file: {}", e));
                std::process::exit(1);
            }

            chalk
                .yellow()
                .bold()
                .println(&format!("💾 Domains saved to: {}", output_path.display()));
        }
        Err(e) => {
            sp.stop_with_message(chalk.red().bold().string(&format!("\n❌ Error: {}", e)));
        }
    }
}
