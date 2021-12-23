# MACC - Cryptocurrency node

This project contains the code for the node and relies on the lib.

## Running

1. Build the project with `cargo build --release`
2. Copy the binary `./target/release/macc(.exe)` to a new folder
3. Create a config in the same folder, example config can be found below
4. Run the binary

### Example Config
```
{
    "port": 8033,
    "address": "address",
    "data_file": "node.bin",
    "trusted_nodes": []
}
```