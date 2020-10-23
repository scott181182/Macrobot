# Macrobot

This is currently a proof-of-concept for a TCP server and web client which can
execute key macros as sent to it.

## Getting Started

A `cargo run` will server the application and website.

If changing the website code. Simply run `npm install` and `npm run build` in 
the project's `website` directory and it'll generate the public files in
`website/public` and `cargo run` will serve the new files.

### Cross Compiling

Ensure you have cross installed (`cargo install cross`) and have the docker
daemon running (`sudo systemctl start docker`). If you are on windows see: 
https://docs.docker.com/docker-for-windows/wsl/

`cross build --target x86_64-pc-windows-gnu`

Move the compiled exe from the `target/x86_64-pc-windows-gnu` folder to the 
project root. Run the exe and you should see the site at `localhost:8073`.

### Linux Dependencies

Linux users may have to install libxdo-dev. For example, on Ubuntu:

`apt install libxdo-dev`

## How to Use

The program currently listens on TCP port 8073. Clients can connect and send
Unicode, line-delimited messages to the server. The server currently support
three commands:
* `exit`: disconnects the client
* `quit`: disconnects the client AND shuts down the server
* `key <key command>`: executes the given key command on the server

All messages are converted to lowercase, so no need to worry about case
sensitivity. So, if you want to type a capital letter, you have to send it with
the `shift` modifier.

### Key Commands

Key commands are of the following syntax:
```
<key-command> ::= (<modifier> "+")* <character>
<modifier> ::= "shift" | "ctrl" | "alt" | "meta"
<character> ::= any keyboard character, probably
```

Examples:
```
m
shift+n
ctrl+c
meta+n
ctrl+shift+p
```
