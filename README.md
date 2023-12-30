# Random Meal SVG Generator

![Screenshot of generated meal svg](https://media.discordapp.net/attachments/1102888096007196733/1190657177820602438/marshallku.dev_48018_.png)

The Random Meal SVG Generator is a web application built with Rust and Axum framework. It fetches random meal data from an external API and renders this data as an SVG image.

## Prerequisites

- Rust
- Docker

### Additional packages

```bash
sudo apt install pkg-config libssl-dev
```

In order to run the application using `cargo run`, the `reqwest` library requires the `pkg-config` and `libssl-dev` packages to be installed

## Run the application

### With Cargo

```bash
cargo run
```

You can run the application directly using Cargo.

### With Docker

```bash
docker build -t menu-today .
docker run -e BIND_ADDRESS=0.0.0.0 -p 41880:41880 --restart=unless-stopped -d menu-today
```

Alternatively, this application can be effortlessly executed using Docker.

### Usage

```txt
http://127.0.0.1:41880/
```

Access the application via a web browser or any HTTP client at the address where the server is running. For example, if running locally on the default port(41880).

### Testing

To run the unit tests:

```bash
cargo test
```

This will execute the tests defined in the application, ensuring all components function as expected.
