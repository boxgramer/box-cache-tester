# 📦 Box Cache Tester

**Box Cache Tester** is a command-line tool built in Rust for testing web cache behavior, especially useful for detecting and testing **cache poisoning** vulnerabilities. It supports customizing request headers, stripping response headers, and detecting reflected content in HTML responses with colorized terminal output.

---
## LAB Test
- [Lab: URL normalization](https://portswigger.net/web-security/web-cache-poisoning/exploiting-implementation-flaws/lab-web-cache-poisoning-normalization)

## 🚀 Build & Run

```bash
cargo build && cargo run -- --url=https://target.com
```

Replace `https://target.com` with the URL of the host you want to test.

---

## 🛠️ Features

- Send dynamic paths and headers.
- Detect if a value is reflected in the response.
- Check cache behavior using `x-cache` or similar headers.
- Modify headers live with shell-like input.
- Colorful terminal output for better visibility.

---

## 💻 Usage Example

```bash
>>> /<script>alert(1)</script> -F="<script>alert(1)</script>"
```

This will:

- Send request to `https://target.com/<script>alert(1)</script>`
- Highlight any reflection of that script in the response.
- Display request and response headers.

---

## 🔧 Commands Inside Prompt

- `-F="<payload>"` → Set reflection payload.
- `-H="Header: Value"` → Add custom header.
- `-R="header_name"` → Remove header by keyword (case-insensitive).
- `exit` or `quit` → Exit the tool.

---

## 📦 Dependencies

- [curl](https://crates.io/crates/curl)
- [regex](https://crates.io/crates/regex)
- [colored](https://crates.io/crates/colored)
- [clap](https://crates.io/crates/clap)
- [rustyline](https://crates.io/crates/rustyline)
- [shell-words](https://crates.io/crates/shell-words)

Install dependencies and run with:

```bash
cargo build
cargo run -- --url=https://target.com
```

---

## 🐛 Example Output

```
Target:https://0a2e00ae03ae0d738041039c004b001f.web-security-academy.net
path : /<script>alert(1)</script>
headers:[]
find: <script>alert(1)</script>
--------------------------------------------------
Request: "https://.../<script>alert(1)</script>"
--------------------------------------------------
"HTTP/2 404"
"x-cache: miss"
...
--------------------------------------------------
<p>Not Found: /<script>alert(1)</script></p>
```

---

## ☠️ Warning

This tool is for educational and authorized testing **only**. Do not use on targets without permission.## 📄 License

MIT © Boxgramer

