# Rust web server

No web frameworks, only vanilla rust and multi thread concepts.
Web server example from rust book.

Practice:

- Change calls to unwrap to more robust error handling.
- Use ThreadPool to perform some task other than serving web requests.
- Add tests of the library’s functionality.

Install

```
cargo build
```

Run

```
cargo run src/main.rs
```

This will start a web server on the desired port and serve multiple requests with a basic response html template.
