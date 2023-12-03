# Today's menu to image

![Screenshot of menu - Hot and Sour Soup](https://cdn.discordapp.com/attachments/1102888096007196733/1180843590608486450/localhost_41880__1.png)

Display a random menu using SVG in Rust

## Requirements

```bash
sudo apt install pkg-config libssl-dev
```

In order to run the application using `cargo run`, the `reqwest` library requires the `pkg-config` and `libssl-dev` packages to be installed

## Docker

```bash
docker build -t menu-today .
docker run -e BIND_ADDRESS=0.0.0.0 -p 41880:41880 --restart=unless-stopped -d menu-today
```

Alternatively, this application can be effortlessly executed using Docker.
