# [M]atura[A]rbeit [C]rypto[C]urrency

This project contains the code for the complete cryptocurrency and a frontend application


## Layout

[lib](lib/README.md) - Rust library with the data structures and helper functions used by the other projects

[bin](bin/README.md) - Contains code for the cryptocurrency node

[py](py/README.md) - Python library for `lib` and a wallet implementation (WARNING: python library is outdated)

[www](www/README.md) - ReactJS Frontend for the node, has an explorer, wallet and faucet


## Requirements

A working installation of:

- [rust nightly](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html)
- [nodejs](https://nodejs.org/en/download/)
- [python](https://www.python.org/downloads/)

is required.



## Usage

How to run each project is described in the README.md located in the project's folder


## Improvements
- Broadcast transactions
- Use `script::eval` in `get_owned_transactions`
- Use a better algorithm in `get_send_body`
- move all functions to macc-lib so www-lib can only expose them
- Re-exports in macc-lib
- Tests
- Update python library
- Colored logging for windows
- Miner handle sigint
- Publish Docker images