# Macrobot

This is currently a proof-of-concept for a TCP server which can execute key macros as sent to it. This is ONLY the
server, so you'll need netcat, telnet, or another similar utility to actually send it commands.

## Building

This tool uses Make (yes I know it should use Ant or Maven) for building. Simply run `npm install` and `npm run build` in the
project's `website` directory and it'll generate the public files in `website/public`.

### Cross Compiling

Ensure you have cross installed (`cargo install cross`) and have the docker
daemon running (`sudo systemctl start docker`). If you are on windows see: 
https://docs.docker.com/docker-for-windows/wsl/

`cross build --target x86_64-pc-windows-gnu`

Move the compiled exe from the `target/x86_64-pc-windows-gnu` folder to the 
project root. Run the exe and you should see the site at `localhost:8073`.


## Native Compilation
```shell
cargo run
```


## How to Use

The program currently listens on TCP port 8073. Clients can connect and send Unicode, line-delimited messages to the
server. The server currently support three commands:
* `exit`: disconnects the client
* `quit`: disconnects the client AND shuts down the server
* `key <key command>`: executes the given key command on the server

All messages are converted to lowercase, so no need to worry about case sensitivity. So, if you want to type a capital
letter, you have to send it with the `shift` modifier.

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
