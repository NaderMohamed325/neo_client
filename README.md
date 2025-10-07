

````markdown
# 🦀 neo_client — Simple HTTP Client CLI Tool

`neo_client` is a lightweight command-line HTTP client built in **Rust**, designed for learning and experimentation with low-level networking using raw **TCP sockets**.  
It lets you send basic HTTP requests (like `GET`) directly from your terminal — no external libraries like `reqwest` needed.

---

## 🚀 Features

✅ Send simple `GET` requests over TCP  
✅ Specify **URL**, **port**, and **route**  
✅ Built-in colorful CLI output  
✅ Graceful error handling  
✅ Simple and fast — built in pure Rust  

---

## 🧩 Example Usage

### 1️⃣ Basic Request
```bash
neo_client --url example.com
````

➡ Sends a `GET /` request to port `80`.

### 2️⃣ With Route

```bash
neo_client --url jsonplaceholder.typicode.com --route /posts/1
```

➡ Fetches `/posts/1` from JSONPlaceholder’s fake REST API.

### 3️⃣ Custom Port

```bash
neo_client --url example.com --port 8080
```

➡ Connects to port `8080` instead of default `80`.

### 4️⃣ Explicit Method

```bash
neo_client --url example.com --method GET
```

➡ Currently only `GET` is supported — more coming soon!

---

## ⚙️ CLI Options

| Flag | Long Option | Description                          | Default      |
| ---- | ----------- | ------------------------------------ | ------------ |
| `-u` | `--url`     | Targeted host (without `http://`)    | **required** |
| `-m` | `--method`  | HTTP method (e.g., GET, POST)        | `GET`        |
| `-p` | `--port`    | Target port number                   | `80`         |
| `-r` | `--route`   | Specific route path (e.g., `/posts`) | `/`          |

---

## 🏗️ Installation

### 1️⃣ Clone the repo

```bash
git clone https://github.com/<your-username>/neo_client.git
cd neo_client
```

### 2️⃣ Build the project

```bash
cargo build --release
```

### 3️⃣ Run it

```bash
./target/release/neo_client --url example.com
```

---

## 🧠 How It Works

`neo_client` uses Rust’s standard library only:

* `std::net::TcpStream` — to connect directly to servers over TCP.
* `std::io::{Read, Write}` — to send HTTP requests and read raw responses.
* `clap` — to handle command-line argument parsing.
* `colored` — for pretty colored output.

Example request built by `neo_client`:

```
GET / HTTP/1.1
Host: example.com
Connection: close
```

---

## 🧰 Tech Stack

* 🦀 **Rust**
* ⚙️ **clap** — CLI argument parsing
* 🎨 **colored** — terminal colors

---

## 📦 Future Plans

* [ ] Support `POST`, `PUT`, `DELETE`
* [ ] Allow sending JSON bodies (`--body`)
* [ ] Display response headers/body separately
* [ ] Add timeout and error recovery
* [ ] Pretty-print JSON responses

---


## 🪪 License

This project is licensed under the **MIT License** — feel free to use, modify, and share.

---

### ⭐ If you like this project, consider giving it a star on GitHub!

