use crossterm::{
    cursor,
    queue,
    style::{Stylize},
    terminal, Result,
};
use regex::Regex;
use std::{
    io::{self, stdout, Write},
    process::{self},
};
mod api;
mod client;
mod config;
mod structs;


fn main() -> Result<()> {
    let mut stdout = stdout();
    ctrlc::set_handler(move || {
        println!("{}", "[X] Ctrl+C detected! Exiting...".red().bold());
        process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");
    let token = config::get_token();
    queue!(
        stdout,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    )?;
    println!("{}", include_str!("banner.txt").blue().bold());
    println!("{} {}", "[+]".blue().bold(), "Select option".cyan());
    let mut n: i32 = 1;
    for x in vec!["Favorite games", "All categories", "Search"] {
        println!("{} {}", format!("[{}]", n).blue().bold(), x.green());
        n += 1;
    }
    drop(n);
    let index = read_int(3);
    println!();
    let mut game: structs::GameFull;
    let quality: String;
    let mut game_code: String;
    loop {
        match index {
            1 => {
                let favgames = api::get_favorite_games(token.to_string());
                println!("{} {}", "[+]".blue().bold(), "Select game".cyan());
                if favgames.game_shipments_any.is_some() {
                    let mut n: i32 = 1;
                    let games: Vec<structs::Game> = favgames.game_shipments_any.unwrap();
                    for x in games.clone() {
                        println!(
                            "{} {}",
                            format!("[{}]", n).cyan().bold(),
                            x.title.unwrap().green()
                        );
                        n += 1;
                    }
                    let code = games[(read_int(n - 1) - 1) as usize].clone().code.unwrap();
                    drop(n);
                    game = api::get_game_by_code(token.to_string(), code.to_string());
                    game_code = game.code.unwrap().to_string();
                    break;
                }
            }
            2 => {
                let categories = api::get_categories(token.to_string());
                println!("{} {}", "[+]".blue().bold(), "Select category".cyan());
                if categories.categories.is_some() {
                    let mut n: i32 = 1;
                    let categoties: Vec<structs::Category> = categories.categories.unwrap();
                    for x in categoties.clone() {
                        println!(
                            "{} {}",
                            format!("[{}]", n).cyan().bold(),
                            x.name.unwrap().green()
                        );
                        n += 1;
                    }
                    let category = categoties[(read_int(n - 1) - 1) as usize]
                        .clone()
                        .code
                        .unwrap();
                    let _games = api::get_games_by_category(token.to_string(), category);
                    println!("{} {}", "[+]".blue().bold(), "Select game".cyan());
                    if _games.game_shipments_any.is_some() {
                        let mut n: i32 = 1;
                        let games: Vec<structs::Game> = _games.game_shipments_any.unwrap();
                        for x in games.clone() {
                            println!(
                                "{} {}",
                                format!("[{}]", n).cyan().bold(),
                                x.title.unwrap().green()
                            );
                            n += 1;
                        }
                        let code = games[(read_int(n - 1) - 1) as usize].clone().code.unwrap();
                        drop(n);
                        game = api::get_game_by_code(token.to_string(), code.to_string());
                        game_code = game.code.unwrap().to_string();
                        break;
                    }
                }
            }
            3 => {
                println!("{} {}", "[+]".blue().bold(), "Type game title".cyan());
                let name = read_string();
                let games = api::get_games_by_name(token.to_string(), name.to_string());
                if games.game_shipments_any.is_some() {
                    let games: Vec<structs::Game> = games.game_shipments_any.unwrap();
                    if games.len() > 0 {
                        let mut n: i32 = 1;
                        for x in games.clone() {
                            println!(
                                "{} {}",
                                format!("[{}]", n).cyan().bold(),
                                x.title.unwrap().green()
                            );
                            n += 1;
                        }
                        let code = games[(read_int(n - 1) - 1) as usize].clone().code.unwrap();
                        drop(n);
                        game = api::get_game_by_code(token.to_string(), code.to_string());
                        game_code = game.code.unwrap().to_string();
                        break
                    }
                }
                println!("{} {}", "[X]".blue().bold(), "No games. Try another search pattern...".yellow());
                continue;
            },
            _ => {}
        }
    }
    println!("\n{} {}", "[+]".blue().bold(), "Game information".cyan());
    println!(
        "{} {}",
        "[+] Title:".cyan().bold(),
        game.title.unwrap_or("null".to_string()).green()
    );
    println!(
        "{} {}\n",
        "[+] Year:".cyan().bold(),
        game.year.unwrap_or(-1).to_string().green()
    );
    if game.child_games.is_some() {
        let platforms = game.child_games.unwrap();
        println!(
            "{} {}",
            "[+]".blue().bold(),
            "Select digital games platform".cyan()
        );
        let mut n: i32 = 1;
        for x in platforms.clone() {
            println!(
                "{} {}",
                format!("[{}]", n).cyan().bold(),
                x.game_platform.unwrap().name.unwrap().green()
            );
            n += 1;
        }
        let index = (read_int(n - 1) - 1) as usize;
        game = serde_json::from_str::<structs::GameFull>(
            &serde_json::to_string(&platforms[index].clone()).unwrap(),
        )
        .unwrap();
        game_code = platforms[index].clone().code.unwrap().to_string();
        drop(n);
    }
    let resolutions = game.run.unwrap();
    println!("{} {}", "[+]".blue().bold(), "Select quality".cyan());
    let mut n: i32 = 1;
    for x in resolutions.clone() {
        println!(
            "{} {}",
            format!("[{}]", n).cyan().bold(),
            format!(
                "{}x{} {}FPS",
                x.parameters.clone().unwrap()[2].clone().value.unwrap(),
                x.parameters.clone().unwrap()[1].clone().value.unwrap(),
                x.parameters.clone().unwrap()[0].clone().value.unwrap()
            )
            .green()
        );
        n += 1;
    }
    quality = resolutions[(read_int(n - 1) - 1) as usize]
        .clone()
        .code
        .unwrap();
    drop(n);
    let game_link = api::get_run_url(
        token.to_string(),
        game_code.to_string(),
        quality.to_string(),
    );
    client::run_client(game_link.to_string());
    Ok(())
}


fn read_string() -> String {
    let re = Regex::new(r"(\r|\n)").unwrap();
    let mut input = String::new();
    print!(" {} ", "->".yellow());
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");
    return String::from(re.replace_all(input.as_str(), "").trim());
}

fn read_int(max_len: i32) -> i32 {
    let re = Regex::new(r"(\r|\n)").unwrap();
    loop {
        let mut input = String::new();
        print!(" {} ", "->".yellow());
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("error: unable to read user input");
        input = String::from(re.replace_all(input.as_str(), ""));
        let my_int = input.parse::<i32>();
        if my_int.is_ok() {
            let int = my_int.unwrap();
            if int > 0 && int <= max_len {
                return int;
            }
        }
        println!("{}", "[X] Need valid int!".red().bold())
    }
}
