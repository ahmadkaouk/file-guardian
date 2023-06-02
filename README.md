# File Guardian

The File guardian project is a Rust-based file transfer system that allows users to upload, download, and verify files over a network. It consists of a server and a client, both of which utilize Merkle trees to ensure the integrity of data during transmission.


> Warning: This project is still in development and is not yet ready for use.

## Features

- **File Upload:** Upload one or more files from your local system to a remote server.
- **File Download:** Download any file previously uploaded to the server.
- **Data Integrity:** Use of Merkle trees to ensure data integrity and allow file verification.
- **Persistent File Management:** The corresponding Merkle tree root hashes are persisted to disk to support data verification upon subsequent downloads.

## Pre-requisites

- [Rust](https://www.rust-lang.org/tools/install)

## Installation

To install the File Guardian project, clone the repository and build the server and client crates:

```bash
$ git clone 
$ cd file-guardian
$ cargo build --release
```

Check out the documentation for the server and client crates for more information on how to use them.

## License

This project is licensed under the MIT license. See the `LICENSE` file for more information.