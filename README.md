# shouldersOfGiants.rs
I have no idea what I'm doing , but llm.c in rust

## Installing Rust

Following the instructions at [Rust-lang](https://www.rust-lang.org/learn/get-started) to install Rust.

## llm.c overview

This repository includes a Makefile to build the C code in llm.c, two python programs containing the test and train data generation which I will also use, and two C programs to  test and train.

I will be converting the C code in llm.c to Rust code in llm.rs.

There is no license in the original repository, so I will need to rewrite the python code. (Correction the original repository is licensed under the MIT license.)

I have some knowledge of C and CUDA, and a tiny bit of Rust, so I expect this to be a challenge.

### Initial notes on the llm.c code

This is an implementation of GPT2 [1] which is a Transformer Decoder model. 

This model consists only of Decoder blocks which consist of:

    - Masked Self-Attention
    - Feed Forward Layer

pairs, stacked.

GPT2 has a context window of 1024 tokens.

The original GPT2 had 12 layers, 768 hidden units, and 12 heads.

I'll be using tiktoken [2] for the tokenizer, eventually. The start of sequence token is ```<s>```. The quickstart version will use the tinyshakespeare dataset, and the tokenizer that Karpathy uses prepro_tinyshakespeare.py.

I don't know yet what the model vocabulary size is. The original GPT2 had a vocabulary size of 50257. The vocabulary matrix which will contain a row for each entry in the vocabulary, padded with 0's to the length of the longest entry. So 50257 rows and embedding size columns.

llm.c uses the included python files to tokenize tinyshakespeare, and I will follow that. 

#### tokenize tinyshakespeare (ty Karpathy)

Download and tokenize a dataset. The [tinyshakespeare](https://raw.githubusercontent.com/karpathy/char-rnn/master/data/tinyshakespeare/input.txt) dataset is the fastest to download and tokenize:

```bash
python prepro_tinyshakespeare.py
```

Because this project uses python I used poetry to manage the python dependencies. 

```bash
poetry init
poetry shell # to activate the virtual environment
poetry install
```

Then running the tokenizer:

```bash
python shouldersofgiants/prepro_tinyshakespeare.py
```
Which created the vocabularty files:
```
Downloading https://raw.githubusercontent.com/karpathy/char-rnn/master/data/tinyshakespeare/input.txt to data/tiny_shakespeare.txt...
data/tiny_shakespeare.txt: 1.06MiB [00:00, 20.1MiB/s]                          
Saved 32768 tokens to data/tiny_shakespeare_val.bin
Saved 305260 tokens to data/tiny_shakespeare_train.bin
```

Looking at the _train.bin file with hexdump:

```
0000000 23f3 0000 0106 0000 99fa 0000 000d 0000
0000010 0274 0000 c450 0000 58db 0000 c038 0000
0000020 0325 0000 0019 0000 00c6 0000 23f3 0000
0000030 0106 0000 99fa 0000 0000 0000 0274 0000
0000040 c450 0000 af4f 0000 0028 0000 0dcf 0000
0000050 061d 0000 0b79 0000 0019 0000 00c6 0000
0000060 a41c 0000 000d 0000 0274 0000 c450 0000
0000070 58db 0000 c038 0000 0325 0000 0019 0000
```
The first thing I noticed is that half of the data is 0's. This is the insight which leads to optimization opportunities. I'll leave that for later, once I get the code working.

## train_gpt2.c review

This implements the training loop for the GPT2 model. which consists of a stack of decoder blocks each of which is made up of a Masked Self-Attention layer and a Feed Forward layer.

Because this is lowlevel code, and not using PyTorch, it will implement both the forward and the backward pass.

### Code outline

main function calls GPT2 model which sets up the model, and loads from a checkpoint. 

### GPT2 model

The GPT2 model is configured with some integers:

    max_seq_len which is max sequence length, e.g. 1024
    vocab_size which is vocab size, e.g. 50257
    num_layers which is number of layers, e.g. 12
    num_heads which is number of heads in attention, e.g. 12
    channels which is number of channels, e.g. 768

So I'll start by creating train_gpt2.rs with an empty main function and then add a GPT2 struct with these fields.


#### Start the Rust project

```bash
cargo init 
```
This complains about the . in the package name (shouldersOfGiants.rs) so I'll use the --name flag.

```bash
cargo init --name shouldersOfGiants
```
which succeeded:
```
Created binary (application) package
```
That created Cargo.toml and src/main.rs I'll keep main.rs as the training code, for now.

To compile the main.rs:

```bash
cd src/
rustc main.rs
```
and then run main

```bash
./main
```
Which outputs "Hello, world!"

Next I'll create the GPT2 config struct.


That worked!

```bash
$ ./main
Max sequence length: 1024
Vocab size: 50257
Number of layers: 12
Number of heads: 12
Number of channels: 768
```

The main data structure in train_gpt2.c is the GPT2 struct which contains the model parameters and the model itself. It is instantiated as a structure called model. I'll write that next.

My first attempt on creating the GPT2 struct in Rust:

```rust
struct GPT2Model {
    config: GPT2Config,
    // other fields
}
...

    let model = GPT2Model {
        config: config,
        // other fields
    };
```

Failed because I'm copying the moving the config struct which is of type GPT2Config, but GPT2Config does not implement Copy. 

I suspect the right thing to do is to move the config struct into the model struct. But, when I consult with the oracle:

```bash
rustc --explain E0382
```

I suspect that moving or copying is not the right plan, and that I should pass the config struct by reference. 

My first guess is:

```rust
    let model = GPT2Model {
        config: &config,
        // other fields
    };
```

That isn't correct. 'expected  `GPT2Config`, found `&GPT2Config`'.
So I'll try:

```rust
    let model = &GPT2Model {
        config: config,
        // other fields
    };
```
That's also not correct and is back to the original error.
I'll move the reference again:

```rust
    let model = GPT2Model {
        &config: config,
        // other fields
    };
```
No, that failes with 'expected identifier, found `&`'.

I'll remove the initial GPT2Config config creation assignment and do the creation within the GPT2Model struct.

```rust
    let model = GPT2Model {
        config: GPT2Config {
            max_seq_len: 1024,
            vocab_size: 50257,
            num_layers: 12,
            num_heads: 12,
            channels: 768,
        };,
        // other fields
    };
``` 
A little tidy up to remove the prior config creation, and the code now works.

```bash
$ ./main
Model config - max sequence length: 1024
Model config - vocab size: 50257
Model config - number of layers: 12
Model config - number of heads: 12
Model config - number of channels: 768
```

[LESSON] The lesson from the config copy error is that it is easier to create something in the correct place than to try to move it around. 

## Day 2 - 2024-04-09 after 693bc1b

A bit of documentation and planning. 

The next step in creating the struct to hold the GPT2 model is to load the checkpoint (gpt2_124M.bin). It took me a while to realize this is created by the reference python implementation train_gpt2.py, which requires PyTorch 2.2 or newer.

So, I'll copy train_gpt2.py from llm.c and create the model checkpoint.

I've reviewed train_gpt2.py and it is lovely code. Attention to detail in the particular form of GeLU used, and some naming nuances. The one change I made was to the location of the data file created out of tokenizing tinyshakespeare. I moved it to the data directory.

To run the python code src/train_gpt2.py:

```bash
poetry init # to activate the virtual environment
python src/train_gpt2.py
```

This failed because I didn't have PyTorch (torch) installed. I'll install it with poetry.

```bash
poetry add torch
```

Once more with the checkpoint build:
```
$ poetry run python src/train_gpt2.py
```

This failed due to missing huggingface transformers. I'll install that with poetry.

```bash
poetry add transformers
```

[1 Illustrated GPT2](https://jalammar.github.io/illustrated-gpt2/)

[2 tiktoken](https://github.com/openai/tiktoken)
