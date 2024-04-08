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


test_gpt2.c
train_gpt2.c

[1 Illustrated GPT2](https://jalammar.github.io/illustrated-gpt2/)

[2 tiktoken](https://github.com/openai/tiktoken)
