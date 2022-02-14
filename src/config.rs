use crossterm::style::Stylize;
use ini::Ini;
use std::{
    process::{self},
    path::Path,
};

pub fn get_token() -> String {
    let _cfg = "playkey.ini";
    if Path::new(&_cfg).exists() {
        let conf = Ini::load_from_file("playkey.ini").unwrap();
        return String::from(conf.section(Some("Playkey")).unwrap().get("token").unwrap());
    } else {
        let mut conf = Ini::new();
        conf.with_section(Some("Playkey"))
        .set("token", "token here...");
        conf.write_to_file(&_cfg).unwrap();
        println!("{} {}", "[+]".blue().bold(), "Config `playkey.ini` created! Type token in config".cyan());
        process::exit(1);
    }
    
}
