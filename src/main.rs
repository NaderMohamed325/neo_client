use clap::{Arg, Command};
use colored::*;
use serde_json::{Value, Map};
use std::io::{Read, Write};
use std::net::TcpStream;

/// Converts a loose key:value string into valid JSON
fn parse_loose_json(input: &str) -> Result<String, std::io::Error> {
    let mut map = Map::new();

    // Remove surrounding braces if present
    let trimmed = input.trim().trim_start_matches('{').trim_end_matches('}');

    for pair in trimmed.split(',') {
        let mut kv = pair.splitn(2, ':');
        let key = kv
            .next()
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid key"))?
            .trim();
        let value = kv
            .next()
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid value"))?
            .trim();

        // Insert as string
        map.insert(key.to_string(), Value::String(value.to_string()));
    }

    Ok(serde_json::to_string(&map)?)
}

fn send_request(
    mut stream: TcpStream,
    method: &str,
    host: &str,
    port: &str,
    route: &str,
    body: Option<&str>,
) -> std::io::Result<()> {
    // Build request headers
    let mut request = format!(
        "{} {} HTTP/1.1\r\nHost: {}:{}\r\nConnection: close\r\n",
        method, route, host, port
    );

    // Add body if present
    if let Some(b) = body {
        // Convert loose JSON to valid JSON
        let body_string = parse_loose_json(b)?;
        let body_bytes = body_string.as_bytes();

        request.push_str(&format!(
            "Content-Type: application/json\r\nContent-Length: {}\r\n\r\n",
            body_bytes.len()
        ));
        request.push_str(&body_string);
    } else {
        request.push_str("\r\n");
    }

    // Debug: print raw request
    println!("{}:\n{}", "Request Sent".bold().yellow(), request);

    // Send request
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
        .version("1.3")
        .author("Nader")
        .about("HTTP client CLI tool with auto JSON formatting")
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
    send_request(stream, &method, url, port, route, body)?;

    Ok(())
}
