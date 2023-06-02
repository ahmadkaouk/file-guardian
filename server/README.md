# File Transfer Server

The File Transfer Server is a Rust server for storing and serving files over a network. It utilizes Merkle trees to ensure the integrity of data during transmission, allowing the server to verify the authenticity and integrity of each file it serves.

## Features

- **File Upload:** Accept file uploads from clients and store them on the server.
- **File Download:** Serve files to clients upon request.
- **Data Integrity:** Use of Merkle trees to ensure data integrity and allow file verification.
- **Persistent File Management:** The corresponding Merkle tree root hashes are persisted to disk to support data verification upon subsequent downloads.


## Usage

Here's how to use the File Transfer Server:

### Starting the Server

To start the server, navigate to the root directory of the server crate and use Cargo to run it:

```bash
$ cd /path/to/server
$ cargo run --release
```


## License

This server is licensed under the MIT license. See the `LICENSE` file for more information.

## Contributing

Contributions are welcome! Please see the `CONTRIBUTING` file for more information.

I hope this helps! Let me know if you have any further questions.