# File Transfer Client

The File Transfer Client is a command-line tool for uploading, downloading, and verifying files from a remote server, written in Rust. It utilizes Merkle trees to ensure the integrity of data during transmission, allowing the client to verify the authenticity and integrity of each file it retrieves.

## Features

- **File Upload:** Upload one or more files from your local system to a remote server.
- **File Download:** Download any file previously uploaded to the server.
- **Data Integrity:** Use of Merkle trees to ensure data integrity and allow file verification.
- **Persistent File Management:** The corresponding Merkle tree root hashes are persisted to disk to support data verification upon subsequent downloads.

## Usage

Here's how to use the File Transfer Client:

### Building the Client

To build the client, navigate to the root directory of the client crate and use Cargo to build it:

```bash
$ cd /path/to/client
$ cargo build --release
```

### Uploading Files

To upload files, invoke the `upload` command, followed by the path to each file:


```bash
./target/debug/client upload -h
Upload one or more files(s) to the server

Usage: client upload --files <FILE>

Options:
  -f, --files <FILE>
  -h, --help           Print help
```

### Downloading Files

To download a file from the server, use the `download` command followed by the name of the file:

```bash
$ ./target/release/client download file1.txt
```

### Listing Files

To view a list of uploaded files, use the `list` command:

```bash
./target/debug/client list --help
List all the uploaded files

Usage: client list

Options:
  -h, --help  Print help
```

## Examples

Suppose you want to upload two files to the server:

```bash
$ ./target/release/client upload -f ~/Documents/file1.txt -f ~/Documents/file2.txt
```

You should see a success message along with the root hash for the uploaded files.

If you want to download `file1.txt` later, you can do it like this:

```bash
$ ./target/release/client download file1.txt
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