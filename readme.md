# Social Media Web Application based on Rust - WASM

### cargo used :

- actix-web for creating web server.
- tera for integrating html files into server

### node dependencies :

- tailwing for styling

### other tech can be explored :

- htmx
- database psql

### quickstart

run

```
cargo build && npm i
```

to build tailwind style and watch changes run :

```
npm run css::build

```

to run the server and watch changes :

install the watchexec
[watchexec](https://github.com/watchexec/watchexec) cli tool.

and then run it :

```
watchexec -e rs -r cargo run
```
