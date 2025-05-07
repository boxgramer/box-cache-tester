use clap::Parser;
use colored::*;
use regex::Regex;
use reqwest::blocking::Client;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Url,
};

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

    #[arg(short = 'F', long)]
    reflect: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut headers: HeaderMap = HeaderMap::new();
    let mut reflect: Option<String> = None;
    let host = cli
        .url
        .ok_or_else(|| Box::<dyn std::error::Error>::from("please insert target url"))?;

    let mut rl = rustyline::DefaultEditor::new()?;

    loop {
        let readline = rl.readline(">>>");

        match readline {
            Ok(readline) => {
                let _ = rl.add_history_entry(readline.as_str());

                println!("Target:{}", host);
                let (status, path) = add_input(readline, &mut reflect, &mut headers)?;
                if !status {
                    break;
                }
                devider();
                println!("Request:");
                devider();
                let (html, headers) = send_request(host.clone(), path, headers.clone())?;
                devider();
                for (key, value) in headers.iter() {
                    println!("{:?},{:?} ", key, value)
                }
                devider();
                let foundhtml = matching(html, reflect.clone().unwrap_or_else(|| "".to_string()));
                println!("{}", foundhtml);
                devider();
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
) -> Result<(bool, String), Box<dyn std::error::Error>> {
    let input = readline.trim();

    if input == "quit" {
        println!("appliaction exit");
        return Ok((false, "".to_string()));
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
        // println!("header : {}", header);
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
        // println!("remove header: {} ", remove);
        headers.remove(remove);
    }

    // println!("input : {}", input);
    // println!("args : {:?}", args);
    println!("path : {}", path_url);
    println!("headers:{:?}", headers);
    println!(
        "find: {}",
        match &reflect {
            Some(s) => s.as_str(),
            None => "",
        }
    );

    Ok((true, path_url.to_string()))
}

fn send_request(
    host: String,
    path: String,
    headers: HeaderMap,
) -> Result<(String, HeaderMap), Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = Url::parse(format!("https://{}{}", host, path).as_str())?;
    let res = client.get(url).headers(headers).send()?;

    let headers = res.headers().clone();
    let html = res.text()?;

    Ok((html, headers))
}

fn matching(html: String, reflect: String) -> String {
    let found = html.find(&reflect);
    match found {
        Some(_) => html.replace(&reflect, &reflect.red().to_string()),
        None => html,
    }
}

fn devider() {
    println!("{}", "-".repeat(50))
}
