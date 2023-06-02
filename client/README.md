# File Transfer Client

The File Transfer Client is a command-line tool for uploading, downloading, and verifying files from a remote server, written in Rust. It utilizes Merkle trees to ensure the integrity of data during transmission, allowing the client to verify the authenticity and integrity of each file it retrieves.

## Features

- **File Upload:** Upload one or more files from your local system to a remote server.
- **File Download:** Download any file previously uploaded to the server.
- **Data Integrity:** Use of Merkle trees to ensure data integrity and allow file verification.
- **Persistent File Management:** The corresponding Merkle tree root hashes are persisted to disk to support data verification upon subsequent downloads.

## Usage

Here's how to use the File Transfer Client:

### Overview

The File Transfer Client is a command-line tool. It has three commands: `upload`, `download`, and `list`. The `upload` command is used to upload one or more files to the server. The `download` command is used to download a file from the server. The `list` command is used to list all the files that have been uploaded to the server.

```bash
$ cargo run --bin client help
    Finished dev [unoptimized + debuginfo] target(s) in 0.38s
     Running `target/debug/client help`
A Cli client to upload/download files to a server and verify their integrity

Usage: client <COMMAND>

Commands:
  list      List all the uploaded files
  upload    Upload one or more files(s) to the server
  download  Download a file from the server
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

Multiple files can be uploaded to the server in batches. The client computes the Merkle tree for each batch of files and persist the root hashes in a json file `uploads.json`, allowing it to verify the integrity of the files it downloads. The root hashes are also used to identify the files that have been uploaded to the server.

To download a file, we provide the name of the file we want to download along with the root hash of the batch of files that contains the file we want to download. The client uses the root hash to retrieve the file from the server and verify the integrity of the file. Files are stored locally in the `downloads` directory.

### Building the Client

To build the client, navigate to the root directory of the client crate and use Cargo to build it:

```bash
$ cd /path/to/client
$ cargo build --release
```

### Uploading Files

To upload files, invoke the `upload` command, followed by the path to each file, and specify the address of the websocket server:

```bash
$ ./target/debug/client upload -h
Upload one or more files(s) to the server

Usage: client upload [OPTIONS] --files <FILE>

Options:
  -f, --files <FILE>
  -s, --server-addr <SERVER_ADDR>  The websocket server address [default: 127.0.0.1:2345]
  -h, --help                       Print help
  ```

### Downloading Files

To download a file from the server, use the `download` command:

```bash
$ ./target/debug/client upload -h
Download a file from the server

Usage: client download [OPTIONS] --file <FILE> --root-hash <ROOT_HASH>

Options:
  -f, --file <FILE>
  -s, --server-addr <SERVER_ADDR>  The websocket server address [default: 127.0.0.1:2345]
  -r, --root-hash <ROOT_HASH>      The root hash of the collection of files where the file is located
  -h, --help                       Print help
```

### Listing Files

To view a list of uploaded files, use the `list` command:

```bash
$ ./target/debug/client list --help
List all the uploaded files

Usage: client list

Options:
  -h, --help  Print help
```

## Examples

Suppose you want to upload two files to the server:

```bash
$ ./target/release/client upload -f ~/Documents/file1.txt -f ~/Documents/file2.txt -s 127.0.0.1:2345
```

You should see a success message along with the root hash for the uploaded files.

If you want to download `file1.txt` later, you can do it like this:

```bash
$ ./target/release/client download --root-hash 96b2874ed9d2cb4d68156d68e3dffa0998d2f7cd17855394e9928cd02ddbd7e4 --file file3
```

If you want to list all the files that have been uploaded to the server, you can do it like this:

```bash
$ ./target/debug/client list
Root hashes and files:
  612ec896f1e05f0a763eae0d052fd1f704e66a996277e35a8a6fa2dea01302d6: ["file3", "file1", "file2"]
  2dba5dbc339e7316aea2683faf839c1b7b1ee2313db792112588118df066aa35: ["file4", "file5"]
```

## Testing

The client includes a suite of unit tests. You can run them with:

```bash
$ cargo test
```

## Contributions

Contributions are welcome!