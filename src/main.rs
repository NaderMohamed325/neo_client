use clap::{Arg, Command};
use colored::*;
use serde_json::Value;
use std::io::{Read, Write};
use std::net::TcpStream;

fn send_request(
    mut stream: TcpStream,
    method: &str,
    host: &str,
    route: &str,
    body: Option<&str>,
) -> std::io::Result<()> {
    // Build request headers
    let mut request = format!(
        "{} {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n",
        method, route, host
    );

    // Add content type + content length for POST/PUT
    if let Some(b) = body {
        request.push_str(&format!(
            "Content-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            b.len(),
            b
        ));
    } else {
        request.push_str("\r\n");
    }

    // Send the request
    stream.write_all(request.as_bytes())?;

    // Read and display response
    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    let parts: Vec<&str> = response.splitn(2, "\r\n\r\n").collect();
    let head = parts[0];
    let body = parts.get(1).unwrap_or(&"");

    println!("{}", "Response Headers:".bold().yellow());
    println!("{}\n", head.blue());

    println!("{}", "Response Body:".bold().yellow());
    match serde_json::from_str::<Value>(body) {
        Ok(json_val) => {
            let formatted = serde_json::to_string_pretty(&json_val).unwrap();
            println!("{}", formatted.green());
        }
        Err(_) => println!("{}", body),
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("neo")
        .version("1.1")
        .author("Nader")
        .about("HTTP client CLI tool")
        .arg(
            Arg::new("url")
                .short('u')
                .long("url")
                .help("Target host (without http://)")
                .num_args(1)
                .required(true),
        )
        .arg(
            Arg::new("method")
                .short('m')
                .long("method")
                .help("HTTP method (GET, POST, PUT, DELETE)")
                .num_args(1)
                .default_value("GET"),
        )
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .help("Server port")
                .num_args(1)
                .default_value("80"),
        )
        .arg(
            Arg::new("route")
                .short('r')
                .long("route")
                .help("API route (e.g., /api/users)")
                .num_args(1)
                .default_value("/"),
        )
        .arg(
            Arg::new("body")
                .short('b')
                .long("body")
                .help("JSON body for POST/PUT requests")
                .num_args(1),
        )
        .get_matches();

    // Extract arguments
    let url = matches.get_one::<String>("url").unwrap();
    let method = matches.get_one::<String>("method").unwrap().to_uppercase();
    let port = matches.get_one::<String>("port").unwrap();
    let route = matches.get_one::<String>("route").unwrap();
    let body = matches.get_one::<String>("body").map(|s| s.as_str());

    let address = format!("{}:{}", url, port);

    // Connect to server
    let stream = TcpStream::connect(&address)?;
    println!("{}", "Connected to the server!".green());

    // Send request
    send_request(stream, &method, &url, &route, body)?;

    Ok(())
}
