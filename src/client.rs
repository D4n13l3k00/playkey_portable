use crossterm::{
    style::{Stylize},
};
use std::{
    io::{self, Cursor},
    process::{self, Command},
};
use uuid::Uuid;

pub fn run_client(link: String) {
    let mut path = std::env::current_dir().unwrap();
    let client_name = format!("client-{}.exe", Uuid::new_v4());
    path = path.join(&client_name);
    {
        let mut archive = zip::ZipArchive::new(Cursor::new(include_bytes!("packed.zip"))).unwrap();
        println!(
            "{} {}",
            "[+]".blue().bold(),
            "Unpacking client...".green().bold()
        );
        let mut file = match archive.by_name("_") {
            Ok(file) => file,
            Err(..) => {
                println!("{}", "[X] Error... I can't unpack client...".red().bold());
                process::exit(1);
            }
        };
        io::copy(&mut file, &mut std::fs::File::create(&path).unwrap()).unwrap();
    }
    println!(
        "{} {}",
        "[+]".blue().bold(),
        "Executing client...".green().bold()
    );
    let c = Command::new(&path).arg(link).output();
    if c.is_ok() {
        println!(
            "{} {}",
            "[+]".blue().bold(),
            format!(
                "Client exited with code {:?}! Deleting it...",
                c.unwrap().status.code().unwrap()
            )
            .green()
            .bold()
        );
    } else {
        println!(
            "{} {}",
            "[X]".blue().bold(),
            "Client exited strangely (unexcepted error!)".red().bold()
        );
        println!("{}", format!("{:#?}", c.unwrap_err()).red());
    }
    std::fs::remove_file(&path).unwrap();
}