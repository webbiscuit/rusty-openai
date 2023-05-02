use anyhow::Result;
use colored::Colorize;
use dotenv::dotenv;
use std::io::Write;
use std::{env, io};
use textwrap::{termwidth, Options};

mod bot;
use bot::Bot;

fn print_header() {
    println!(
        "{}",
        r" ___  ___  __                             __            ___ ".on_green()
    );
    println!(
        "{}",
        r"  |  |__  |__)  |\/| | |\ |  /\  |       /  ` |__|  /\   |  ".on_green()
    );
    println!(
        "{}",
        r"  |  |___ |  \  |  | | | \| /~~\ |___    \__, |  | /~~\  |  ".on_green()
    );
    println!(
        "{}",
        r"                                                            ".on_green()
    );
    // println!("{}", "TermChat".green());
    println!("{}", "A terminal client for OpenAI's Chat APIs".yellow());
}

fn get_user_input() -> Result<String> {
    io::stdout().flush().unwrap();
    let mut prompt = String::new();
    std::io::stdin().read_line(&mut prompt)?;

    Ok(prompt.trim().to_string())
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not found");
    let mut bot = Bot::new(api_key);

    print_header();

    loop {
        print!("YOU {} ", ">".green());
        let user_input = get_user_input()?;

        let bot_response = bot.get_response(&user_input).await?;

        const BOT_NAME_LENGTH: usize = 6;
        let bot_output = textwrap::fill(
            &bot_response,
            Options::new(termwidth() - BOT_NAME_LENGTH)
                .subsequent_indent(&" ".repeat(BOT_NAME_LENGTH)),
        );

        println!("BOT {} {}", ">".green(), bot_output.blue());
    }

    //Ok(())
}
