use std::io::{self, Write};

use clap::Parser;
use regex::Regex;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use rustyline::Editor;

#[derive(Parser)]
#[command(version = "1.0", about = "Request Cache Tester")]
struct Cli {
    #[arg(short, long)]
    url: Option<String>,
}

#[derive(Parser)]
#[command(version = "1.0", disable_help_flag = true)]
struct CommandArg {
    #[arg(short = 'H', long)]
    header: Option<String>,

    #[arg(short = 'R', long)]
    remove: Option<String>,

    #[arg(short = 'f', long)]
    reflect: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut headers: HeaderMap = HeaderMap::new();
    let mut reflect: Option<String> = None;
    let host = cli
        .url
        .ok_or_else(|| Box::<dyn std::error::Error>::from("please insert target url"))?;

    println!("Target:{}", host);

    let mut rl = rustyline::DefaultEditor::new()?;

    loop {
        let readline = rl.readline(">>>");

        match readline {
            Ok(readline) => {
                let _ = rl.add_history_entry(readline.as_str());

                let status = add_input(readline, &mut reflect, &mut headers)?;
                if !status {
                    break;
                }
            }
            Err(_) => println!("No input"),
        }
    }

    Ok(())
}
fn add_input(
    readline: String,
    reflect: &mut Option<String>,
    headers: &mut HeaderMap,
) -> Result<bool, Box<dyn std::error::Error>> {
    let input = readline.trim();

    if input == "quit" {
        println!("appliaction exit");
        return Ok(false);
    }

    let args = shell_words::split(input)?;
    let path_url = if let Some(path) = args.first() {
        path
    } else {
        ""
    };

    let command = CommandArg::try_parse_from(&args).unwrap_or_else(|e| {
        eprintln!("argument invalidd :{:?}", e);
        std::process::exit(1)
    });
    *reflect = command.reflect;

    if let Some(header) = command.header {
        println!("header : {}", header);
        let rgx = Regex::new(r"^(.*?):\s*(.*)$").unwrap();
        if let Some(cap) = rgx.captures(&header) {
            let left = cap.get(1).unwrap().as_str();
            let right = cap.get(2).unwrap().as_str();
            headers.insert(
                HeaderName::from_bytes(left.as_bytes())?,
                HeaderValue::from_str(right)?,
            );
        }
    } else {
        println!("header none")
    }

    if let Some(remove) = command.remove {
        println!("remove header: {} ", remove);
        headers.remove(remove);
    }

    println!("input : {}", input);
    println!("args : {:?}", args);
    println!("path : {}", path_url);
    println!("headers:{:?}", headers);
    println!(
        "reflect: {}",
        match &reflect {
            Some(s) => s.as_str(),
            None => "",
        }
    );

    let client = reqwest::Client::new();
    Ok(true)
}
