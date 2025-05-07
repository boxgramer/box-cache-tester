# 📦 Box Cache Tester

**Box Cache Tester** is a command-line tool built in Rust for testing web cache behavior, especially useful for detecting and testing **cache poisoning** vulnerabilities. It supports customizing request headers, stripping response headers, and detecting reflected content in HTML responses with colorized terminal output.

---

## 🧪 Features

* 🔗 Send HTTP requests to a specified URL
* 🧾 Add custom request headers
* ✂️ Remove headers from the response
* 🧠 Test for cache poisoning by manipulating request/response headers
* 🔍 Detect and highlight reflected strings in HTML (shown in **red** in terminal)

---

## 🛠️ Usage

## 📚 Dependencies

* [`reqwest`](https://docs.rs/reqwest)
* [`clap`](https://docs.rs/clap)
* [`colored`](https://docs.rs/colored)

---

## 📄 License

MIT © Boxgramer

