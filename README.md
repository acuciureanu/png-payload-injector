# PNG Payload Injector

PNG Payload Injector is a command-line tool that allows you to embed custom payloads into PNG files and generate corresponding HTML proofs of concept (PoCs). This tool is designed for security testing, educational purposes, and demonstrating certain types of vulnerabilities.

## The research behind

This was a tool that I wrote in PHP back in 2016 after reading a research paper from Adam Logue's blog. [Revisiting XSS payloads in PNG IDAT chunks](https://www.adamlogue.com/revisiting-xss-payloads-in-png-idat-chunks/)

## 🚨 CAUTION

This tool is for educational and testing purposes only. Do not use it for malicious purposes or against systems you don't own or have explicit permission to test.

## 🌟 Features

- Embed custom payloads (e.g., JavaScript, HTML) into PNG files
- Generate HTML PoCs with the embedded payload
- Customizable PNG dimensions
- Verbose mode for detailed operation logging

## 📋 Prerequisites

- [Rust programming language](https://www.rust-lang.org/tools/install)
- Cargo (usually comes with Rust)

## 🛠 Installation

### Option 1: Download from Releases

You can download precompiled binaries for your operating system from the [Releases](https://github.com/acuciureanu/png-payload-injector/releases) section on GitHub. Simply download the binary for your platform, extract it, and run the `png_payload_injector` executable.

### Option 2: Build from Source

1. Clone the repository:

   ```sh
   git clone https://github.com/acuciureanu/png-payload-injector.git
   cd png-payload-injector
   ```

2. Build the project:

   ```sh
   cargo build --release
   ```

3. The compiled binary will be available in `target/release/png_payload_injector`

## 🚀 Usage

Basic syntax:

```sh
png_payload_injector -l <PAYLOAD> [OPTIONS]
```

### Options

- `-l, --payload <PAYLOAD>`: The payload to embed in the PNG (required)
- `-w, --width <WIDTH>`: Width of the PNG image in pixels [default: 66]
- `-e, --height <HEIGHT>`: Height of the PNG image in pixels [default: 60]
- `-o, --output-png <OUTPUT_PNG>`: Output PNG filename [default: "output.png"]
- `-p, --output-html <OUTPUT_HTML>`: Output HTML PoC filename [default: "poc.html"]
- `-v, --verbose`: Use verbose output
- `-h, --help`: Print help information

### Examples

1. Embed a simple alert:

   ```sh
   png_payload_injector -l "<script>alert('Hello!')</script>"
   ```

2. Create a custom-sized PNG with verbose output:

   ```sh
   png_payload_injector -l "<script>console.log('Test')</script>" -w 100 -e 100 -v
   ```

3. Specify custom output files:

   ```sh
   png_payload_injector -l "<img src=x onerror=alert('XSS')>" -o custom.png -p custom.html
   ```

## 📝 Note on Usage

The generated PNG file will appear as a normal image file, but when processed incorrectly by a vulnerable application, it may execute the embedded payload. The HTML PoC demonstrates how the payload could be triggered in a web context.

## 🛡️ Responsible Disclosure

If you discover vulnerabilities using this tool, please practice responsible disclosure:

1. Do not exploit the vulnerability or download unauthorized data.
2. Notify the owner or maintainer of the affected system immediately.
3. Provide sufficient information for the vulnerability to be reproduced and verified.
4. Allow reasonable time for the vulnerability to be patched before public disclosure.

## 🤝 Contributing

Contributions to improve PNG Payload Injector are welcome. Please feel free to submit pull requests or create issues for bugs and feature requests.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 📬 Contact

If you have any questions or concerns, please open an issue in this repository.

Remember to use this tool responsibly and ethically!
