# PortScanner - A Simple Rust Network Scanner

## 📌 Description
**PortScanner** is a lightweight and fast **port scanning tool** written in Rust. It scans a list of ports on a target IP or hostname to check for open services.

## 🚀 Features
- **Synchronous scanning** (no async overhead)
- **Customizable target & port list**
- **Timeout handling to avoid long waits**
- **DNS resolution support** (scan hostnames like `scanme.nmap.org`)

## 📦 Installation
1. **Install Rust** (if not installed):
   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. **Clone the repository**:
   ```sh
   git clone https://github.com/yourusername/portscanner.git
   cd portscanner
   ```
3. **Build the project**:
   ```sh
   cargo build --release
   ```

## 🔧 Usage
Run the scanner with:
```sh
cargo run
```

Or execute the compiled binary:
```sh
target/release/portscanner
```

### Example Output:
```
Scanning scanme.nmap.org...
Port 22 is open
Port 80 is open
Port 443 is open
```

## ⚙️ Customization
You can modify the **target IP/hostname** and **ports** inside `src/main.rs`:
```rust
let target_ip = "scanme.nmap.org"; // Change to your target
let ports = vec![22, 80, 443, 8080, 3306]; // Add or remove ports
```

## 📜 License
This project is licensed under the MIT License.

## 🙌 Contributing
Feel free to open issues or submit pull requests! 🚀

