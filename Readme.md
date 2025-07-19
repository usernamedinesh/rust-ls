🦀 RUST-LS

A simple ls-like utility written in Rust
Clean, fast, and minimal — built using 100% safe Rust std libs.

🚀 Features

✅ List files and directories
✅ Show hidden files with -a
✅ Long format view with -l
✅ Displays permissions, size, and modified time
✅ Unix-style flags: -a, -l, -help, -version

⚙️ Usage

```
cargo run              # List visible files
cargo run -- -a        # Include hidden files
cargo run -- -l        # Long format listing
cargo run -- -a -l     # Combine options
cargo run -- -t        # Sort by modification time (newest first)
cargo run -- -help     # Show help
cargo run -- -version  # Version info
```
📁 Output Example (with -l)

```
drwxr-xr-x      0     1725910580 src
-rw-r--r--   1452     1725910590 main.rs
```

📦 Installation

```
git clone https://github.com/usernamedinesh/rust-ls
cd rust-ls
cargo build --release
```

🧱 Tech Stack

    Language: Rust

    Tools: cargo, std::fs, std::env, UNIX_EPOCH

    Platform: Linux / macOS
