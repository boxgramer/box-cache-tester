use std::io::{self, Write};

use clap::Parser;

#[derive(Parser)]
#[command(version = "1.0", about = "Request Cache Tester")]
struct Cli {
    #[arg(short, long)]
    url: Option<String>,
}

#[derive(Parser)]
#[command(version = "1.0")]
struct CommandArg {
    #[arg(long)]
    header: Option<String>,

    #[arg(short, long)]
    reflect: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut headers: Vec<String> = Vec::new();
    let mut reflect: Option<String> = None;
    let host = cli
        .url
        .ok_or_else(|| Box::<dyn std::error::Error>::from("please insert target url"))?;

    println!("Target:{}", host);
    loop {
        print!(">");
        io::stdout().flush().unwrap();

        let mut cmd = String::new();
        io::stdin().read_line(&mut cmd).unwrap();

        let input = cmd.trim();

        if input == "quit" {
            println!("appliaction exit");
            break;
        }

        let mut args = input.split_whitespace().collect::<Vec<&str>>();
        let path_url = args.remove(0);

        let command = CommandArg::try_parse_from(&args)?;

        reflect = command.reflect;
        if let Some(header) = command.header {
            headers.push(header);
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
    }

    Ok(())
}
