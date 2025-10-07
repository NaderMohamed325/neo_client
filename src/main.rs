use clap::{Arg, Command};
use colored::*;
use std::io::{Read, Write};
use std::net::TcpStream;

fn get_request(mut stream: TcpStream, host: &str, route: &str) -> std::io::Result<()> {
    // Build a dynamic HTTP GET request
    let request = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        route, host
    );

    // Send the request
    stream.write_all(request.as_bytes())?;

    // Read and print the response
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    println!("{}", response);

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("neo")
        .version("1.0")
        .author("Nader")
        .about("HTTP client CLI tool")
        .arg(
            Arg::new("url")
                .short('u')
                .long("url")
                .help("Targeted host (without http://)")
                .num_args(1)
                .required(true),
        )
        .arg(
            Arg::new("method")
                .short('m')
                .long("method")
                .help("HTTP method")
                .num_args(1)
                .default_value("GET"),
        )
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .help("Select server port")
                .num_args(1)
                .default_value("80"),
        )
        .arg(
            Arg::new("route")
                .short('r')
                .long("route")
                .help("Specific route path (e.g., /posts or /api/users)")
                .num_args(1)
                .default_value("/"),
        )
        .get_matches();

    // Extract arguments
    let url = matches.get_one::<String>("url").cloned().unwrap();
    let method = matches.get_one::<String>("method").cloned().unwrap();
    let port = matches.get_one::<String>("port").cloned().unwrap();
    let route = matches.get_one::<String>("route").cloned().unwrap();

    // Combine host + port for connection
    let address = format!("{}:{}", url, port);

    // Connect to the server
    let stream = TcpStream::connect(&address)?;
    println!("{}", "Connected to the server!".green());

    // Handle methods
    match method.as_str() {
        "GET" => get_request(stream, &url, &route)?,
        _ => eprintln!("{}", "Unsupported method".red()),
    }

    Ok(())
}
