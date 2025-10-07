# 🛰️ Neo Client — A Simple HTTP CLI in Rust

**Neo Client** is a lightweight and colorful HTTP command-line client written in **Rust**.  
It lets you send **GET**, **POST**, **PUT**, and **DELETE** requests directly from your terminal — perfect for testing APIs without needing external tools like Postman or curl.

---

## 🚀 Features

- Supports **GET**, **POST**, **PUT**, and **DELETE**
- Simple CLI interface built with [`clap`](https://docs.rs/clap/latest/clap/)
- Colorized output with [`colored`](https://docs.rs/colored/)
- JSON pretty-printing via [`serde_json`](https://docs.rs/serde_json/)
- Lightweight and fast — built from scratch using TCP sockets
- Graceful fallback for non-JSON responses

---

## ⚙️ Installation

### 1️⃣ Clone the repository

```bash
git clone https://github.com/yourusername/neo_client.git
cd neo_client
```

### 2️⃣ Build the binary

```bash
cargo build --release
```

### 3️⃣ Run it

```bash
./target/release/neo_client --help
```

---

## 🧠 Usage

```bash
neo_client [OPTIONS]
```

### 📋 Options

| Flag           | Description                                  | Default | Example              |
| -------------- | -------------------------------------------- | ------- | -------------------- |
| `-u, --url`    | Target host (without `http://`)              | —       | `127.0.0.1`          |
| `-m, --method` | HTTP method (`GET`, `POST`, `PUT`, `DELETE`) | `GET`   | `POST`               |
| `-p, --port`   | Server port                                  | `80`    | `8000`               |
| `-r, --route`  | API route or endpoint                        | `/`     | `/api/users`         |
| `-b, --body`   | JSON body for `POST`/`PUT`                   | —       | `'{"name":"nader"}'` |

---

## 💡 Examples

### 🔍 GET Request

```bash
neo_client --url 127.0.0.1 -p 8000 -m GET -r /api
```

**Output:**

```
Connected to the server!
Response Headers:
HTTP/1.1 200 OK
Content-Type: application/json; charset=utf-8

Response Body:
{
    "status": "success",
    "data": {
        "name": "nader"
    }
}
```

---

### ➕ POST Request

```bash
neo_client --url 127.0.0.1 -p 8000 -m POST -r /api/users -b '{"name":"nader"}'
```

---

### 📝 PUT Request

```bash
neo_client --url 127.0.0.1 -p 8000 -m PUT -r /api/users/1 -b '{"name":"updated name"}'
```

---

### ❌ DELETE Request

```bash
neo_client --url 127.0.0.1 -p 8000 -m DELETE -r /api/users/1
```

---

## 🧩 Example Internal Flow

1. The CLI parses arguments using `clap`
2. Opens a raw TCP connection with `TcpStream`
3. Builds the HTTP request manually:

   ```
   METHOD /route HTTP/1.1
   Host: example.com
   Content-Type: application/json
   Content-Length: N
   ```

4. Sends it through the socket
5. Reads the response and:

   - Splits headers and body
   - Pretty-prints JSON responses
   - Colors headers (blue) and JSON (green)

---

## 🧰 Dependencies

| Crate                                       | Purpose                     |
| ------------------------------------------- | --------------------------- |
| [`clap`](https://docs.rs/clap/)             | CLI argument parsing        |
| [`colored`](https://docs.rs/colored/)       | Terminal color formatting   |
| [`serde_json`](https://docs.rs/serde_json/) | JSON parsing and formatting |

---

## 🧑‍💻 Author

**Nader**
Rust developer passionate about systems programming, embedded systems, and backend development.

---

## 📄 License

This project is licensed under the **MIT License** — free to use, modify, and distribute.

---

## ⭐️ Future Enhancements

- [ ] Support for HTTPS (via `rustls`)
- [ ] File upload support (`multipart/form-data`)
- [ ] Save and load request profiles
- [ ] Response time and latency metrics
- [ ] Pretty colorized JSON keys and values
- [ ] Support for Auth headers (Bearer, Basic)

---

### 💬 Example Summary

✅ Simple
✅ Fast
✅ Educational

A perfect project to understand how HTTP works under the hood — without any heavy dependencies.
