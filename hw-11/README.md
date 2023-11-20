# Homework for Lesson #9 - Network I/O

## Problem
1. Create a server that allows connections at specified `port` and `hostname`.
2. Create a client that also accepts `port` and `hostname`.
3. The client has the collowing commands: `.file <path>`, `.image <path>`, `.quit`.
4. The server should save files to `files/` and images to `images/`.
5. The server should convert images to `.png` before saving them .

## Approach
The project is split into three crates: client, server, and common.

- `client`: Handles user input and sends commands to the server.
- `server`: Listens for client connections and processes incoming data.
- `common`: Shared code.

Utilizes `Tokio`` for asynchronous I/O operations and `serde_cbor`` to serialize and deserialize messages.

## Usage
Client-side commands to test different functions:

#### Send files
```.file ../test/trpl2.pdf```

#### Send images
```.image ../test/nice.png```

```.image ../test/mri.jpg```

#### Send text
```Crabs can walk in all directions, but mostly walk and run sideways.```