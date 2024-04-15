# Workflows

## activate the virtual environment

```bash
poetry shell
```

## build the project

```bash
# from the root of the project
cargo build
```

## run the code

```bash
# from the root of the project
cargo run
```

## look at the values in a binary file

```bash
# in the same directory as the binary file
hexdump -C gpt2_124M.bin
```
```
00000000  c6 d7 34 01 01 00 00 00  00 04 00 00 51 c4 00 00  |..4.........Q...|
00000010  0c 00 00 00 0c 00 00 00  00 03 00 00 00 00 00 00  |................|
00000020  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
```
