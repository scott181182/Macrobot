# Macrobot

This is currently a proof-of-concept for a TCP server which can execute key macros as sent to it. This is ONLY the
server, so you'll need netcat, telnet, or another similar utility to actually send it commands.

## Building

This tool uses Make (yes I know it should use Ant or Maven) for building. Simply run `make` or `make build` in the
project directory and it'll generate a `Macrobot.jar` file.

The entry point to the program is in `src/sgf/App.java`.

## Running

Running can be done with make or with java directly:
```shell
make run
```
```shell
java -jar Macrobot.jar
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
