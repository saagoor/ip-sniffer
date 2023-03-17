# IP Sniffer
A simple IP sniffer written in Rust.
It scans the ports of a given IP address and prints the open ones.

## Usage
```
cargo run -- -j <Max threads> <IP address>
```
The `-j` flag specifies the number of threads to use.

## Example
```
cargo run -- -j 100 127.0.0.0
```
This will scan the IP range from 1 to 65535 using 100 threads.

## Notes
This is a very simple program and it is not optimized for speed.
It is just a practice project to learn Rust.
