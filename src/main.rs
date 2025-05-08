use std::sync::{Arc, Mutex};

use clap::Parser;
use colored::*;
use curl::easy::{Easy, List};
use regex::Regex;

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
    let mut headers: Vec<String> = Vec::new();
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
                let (html, url, headers) = send_request(host.clone(), path, headers.clone())?;
                println!("Request: {:?}", url.as_str());
                devider();
                devider();

                for value in headers.iter() {
                    println!("{:?} ", value);
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
    headers: &mut Vec<String>,
) -> Result<(bool, String), Box<dyn std::error::Error>> {
    let input = readline.trim();

    if input == "quit" || input == "exit" {
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
            headers.push(format!("{}: {}", left, right));
        }
    } else {
        println!("header none")
    }

    if let Some(remove) = command.remove {
        // println!("remove header: {} ", remove);
        headers.retain(|header| {
            if let Some((key, _)) = header.split_once(':') {
                !key.to_lowercase().contains(&remove)
            } else {
                true
            }
        });
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
    headers: Vec<String>,
) -> Result<(String, String, Vec<String>), Box<dyn std::error::Error>> {
    let mut easy = Easy::new();
    let url = format!("{}{}", host, path);
    easy.url(url.as_str())?;

    let mut request_header = List::new();
    for header in headers.iter() {
        request_header.append(header.as_str())?;
    }
    easy.http_headers(request_header)?;

    let response_header = Arc::new(Mutex::new(Vec::new()));
    let response_header_clone = Arc::clone(&response_header);
    easy.header_function(move |header| {
        let line = String::from_utf8_lossy(header).trim().to_string();
        if let Ok(mut rheader) = response_header_clone.lock() {
            rheader.push(line);
        }
        true
    })?;

    let body = Arc::new(Mutex::new(Vec::new()));
    let body_clone = Arc::clone(&body);
    easy.write_function(move |data| {
        if let Ok(mut body) = body_clone.lock() {
            body.extend_from_slice(data);
        }
        Ok(data.len())
    })?;

    easy.perform()?;

    let html = body.lock().unwrap();
    let html_body = String::from_utf8_lossy(&html);
    let result_header = response_header.lock().unwrap().clone();

    Ok((html_body.to_string(), url, result_header))
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
