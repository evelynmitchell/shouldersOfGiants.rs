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

And then run the checkpoint build again:

```bash
$ poetry run python src/train_gpt2.py
```
That worked!

```
wrote gpt2_124M.bin
wrote gpt2_124M_debug_state.bin
iteration 0, loss: 5.269998550415039
iteration 1, loss: 4.059630870819092
iteration 2, loss: 3.3749470710754395
iteration 3, loss: 2.8005878925323486
iteration 4, loss: 2.3152737617492676
iteration 5, loss: 1.8489487171173096
iteration 6, loss: 1.3945865631103516
iteration 7, loss: 0.9988802075386047
iteration 8, loss: 0.6241909265518188
iteration 9, loss: 0.37664344906806946
<|endoftext|>"If we want to die, we have to die at the front" —
```

## Day 3 - 2024-04-10 after e030079

Added the two files gpt2_124M.bin and gpt2_124M_debug_state.bin to the .gitignore file.

Updated the todo list. 

Verified my dev environment is working by builing and running the main.rs file.

Loading a file from disk in Rust.
Figured out how to do that by reading [3] on IO in the Rust standard library.

Then I got more than a little bit stuck on how to incorporate the function which reads the
checkpoint file into the GPT2Model struct. The struct is defined as:

```rust
struct GPT2Model {
    config: GPT2Config,
    gpt2_build_from_checkpoint: fn() -> io::Result<()>,
    // other fields
}
```
Which is not the full struct, but only my guess so far.

The struct merely describes the function signature. I'm using the main function to instantiate it:
```rust
    let model = GPT2Model {
        config: GPT2Config {
            max_seq_len: 1024,
            vocab_size: 50257,
            num_layers: 12,
            num_heads: 12,
            channels: 768,
        },
        gpt2_build_from_checkpoint: gpt2_build_from_checkpoint,
    };
```

I'm getting several errors, the first of which is:

```
$ cargo check
    Checking shouldersofgiants v0.1.0 (/home/efm/git/myrepos/shouldersOfGiants.rs)
error: expected one of `,`, `:`, or `}`, found `(`
  --> src/main.rs:47:35
   |
39 |     let model = GPT2Model {
   |                 --------- while parsing this struct
...
47 |         gpt2_build_from_checkpoint(model),
   |         --------------------------^ expected one of `,`, `:`, or `}`
   |         |
   |         while parsing this struct field
   |
help: try naming a field
   |
47 |         gpt2_build_from_checkpoint: gpt2_build_from_checkpoint(model),
   |         +++++++++++++++++++++++++++

error[E0063]: missing field `gpt2_build_from_checkpoint` in initializer of `GPT2Model`
  --> src/main.rs:39:17
   |
39 |     let model = GPT2Model {
   |                 ^^^^^^^^^ missing `gpt2_build_from_checkpoint`

For more information about this error, try `rustc --explain E0063`.
```

The first error says that I'm not instantiating the function which reads the checkpoint file correctly. 

So I'll try the proposed fix, to see if that helps.

That helped a bit. The new code is:
```rust
gpt2_build_from_checkpoint:gpt2_build_from_checkpoint(model),
```
and the new error is:
cargo check
    Checking shouldersofgiants v0.1.0 (/home/efm/git/myrepos/shouldersOfGiants.rs)
error[E0425]: cannot find value `model` in this scope
  --> src/main.rs:44:63
   |
44 |         gpt2_build_from_checkpoint:gpt2_build_from_checkpoint(model),
   |                                                               ^^^^^ not found in this scope

error[E0308]: mismatched types
  --> src/main.rs:44:36
   |
44 |         gpt2_build_from_checkpoint:gpt2_build_from_checkpoint(model),
   |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected fn pointer, found `Result<(), Error>`
   |
   = note: expected fn pointer `fn() -> Result<(), std::io::Error>`
                    found enum `Result<(), std::io::Error>`

Some errors have detailed explanations: E0308, E0425.
For more information about an error, try `rustc --explain E0308`.
```

The error on 'model' at line 44 is correct, because line 44 is within the instantiation of 'model'.

So the call to gpt2_build_from_checkpoint should be different. I'm hardcoding the file name for
simplicity, so that is not a parameter. I'll try removing the parameter, to see if that helps.

That helped quite a bit. The new code is:
```rust
fn gpt2_build_from_checkpoint() -> io::Result<()> {
    let mut f = File::open("gpt2_124M.bin")?;
...

        gpt2_build_from_checkpoint:gpt2_build_from_checkpoint(),
...
```
and the new error is:
```
cargo check
    Checking shouldersofgiants v0.1.0 (/home/efm/git/myrepos/shouldersOfGiants.rs)
error[E0308]: mismatched types
  --> src/main.rs:44:36
   |
44 |         gpt2_build_from_checkpoint:gpt2_build_from_checkpoint(),
   |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected fn pointer, found `Result<(), Error>`
   |
   = note: expected fn pointer `fn() -> Result<(), std::io::Error>`
                    found enum `Result<(), std::io::Error>`

For more information about this error, try `rustc --explain E0308`.
```

So the return type of gpt2_build_from_checkpoint is not correct. I'd change it, but I"m not sure how to create a fn pointer. So that is enough for Day 3.

## Day 4 2024-04-14 after b4491eb


Review the todo list. I'm still at file loading of the checkpoints.

Added a workflows file to remind me of common cli commands.

Using phind to help me get ustuck on my current error, where I left off on day 3.

``` phind
The type mismatch error you're encountering is likely due to the way you're trying to assign the `gpt2_build_from_checkpoint` function to the `GPT2Model` struct. In Rust, when you're assigning a function to a struct field, you need to ensure that the function signature matches exactly what's expected by the struct field. In your case, the `gpt2_build_from_checkpoint` function returns `io::Result<()>`, but you're trying to assign it directly to a struct field without calling it.

To fix this issue, you should call the `gpt2_build_from_checkpoint` function when assigning it to the `GPT2Model` struct. However, since the function returns a `Result`, you need to handle this result appropriately. One way to do this is to use the `unwrap` method for simplicity, but be aware that this will panic if the function returns an `Err`. For a more robust solution, you might want to handle the error explicitly.

Here's how you can modify your `main` function to call the `gpt2_build_from_checkpoint` function and assign its result to the `GPT2Model` struct:

```rust
fn main() {
    // Create a new GPT2Model object
    let model = GPT2Model {
        config: GPT2Config {
            max_seq_len: 1024,
            vocab_size: 50257,
            num_layers: 12,
            num_heads: 12,
            channels: 768,
        },
        gpt2_build_from_checkpoint: gpt2_build_from_checkpoint().unwrap(), // Call the function and unwrap the result
        // other fields
    };

    // Access the fields of the model object
    println!("Model config - max sequence length: {}", model.config.max_seq_len);
    println!("Model config - vocab size: {}", model.config.vocab_size);
    println!("Model config - number of layers: {}", model.config.num_layers);
    println!("Model config - number of heads: {}", model.config.num_heads);
    println!("Model config - number of channels: {}", model.config.channels);
}
```

However, this approach won't work directly because the `gpt2_build_from_checkpoint` function is not a field of the `GPT2Model` struct but a standalone function. If you want to store a function within a struct, you should use a function pointer or a closure, depending on your use case.

Given the current structure of your code, it seems like you might want to call `gpt2_build_from_checkpoint` as part of the initialization process of your `GPT2Model` rather than storing it as a field. If that's the case, you should call this function outside of the struct initialization and handle its result accordingly.

If you need further assistance with how to properly store and call functions within a struct, please let me know!```

Ok I was totally confused about initializing ```GPT2Model```. Phind has advice:

```
To call `gpt2_build_from_checkpoint` as part of the initialization process of the `GPT2Model`, you can modify the `GPT2Model` struct to include a method that calls `gpt2_build_from_checkpoint` during its initialization. This approach involves using an **`impl` block** for `GPT2Model` to define a constructor-like method that sets up the model, including loading the checkpoint.

Here's how you can refactor your code to include a method for initializing the `GPT2Model` that calls `gpt2_build_from_checkpoint`:

1. **Define a new method for `GPT2Model`**: This method will be responsible for initializing the model, including calling `gpt2_build_from_checkpoint`.

2. **Modify the `main` function**: Instead of directly creating a `GPT2Model` instance, you'll call this new method to create and initialize the model.

Here's an example of how you might implement these changes:

```rust
// Assuming GPT2Config and GPT2Model are defined as before

impl GPT2Model {
    // This is a constructor-like method for GPT2Model
    fn new() -> io::Result<Self> {
        // Call gpt2_build_from_checkpoint as part of the initialization
        gpt2_build_from_checkpoint()?;

        // Initialize the model with default or specific configuration
        Ok(GPT2Model {
            config: GPT2Config {
                max_seq_len: 1024,
                vocab_size: 50257,
                num_layers: 12,
                num_heads: 12,
                channels: 768,
            },
            // other fields
        })
    }
}

fn main() {
    // Use the new method to create and initialize the GPT2Model
    let model = match GPT2Model::new() {
        Ok(model) => model,
        Err(e) => {
            eprintln!("Failed to initialize GPT2Model: {}", e);
            return;
        }
    };

    // Access the fields of the model object
    println!("Model config - max sequence length: {}", model.config.max_seq_len);
    println!("Model config - vocab size: {}", model.config.vocab_size);
    println!("Model config - number of layers: {}", model.config.num_layers);
    println!("Model config - number of heads: {}", model.config.num_heads);
    println!("Model config - number of channels: {}", model.config.channels);
}
```

In this example, `GPT2Model::new()` is a method that attempts to initialize a `GPT2Model` instance. It first calls `gpt2_build_from_checkpoint()` to load the checkpoint. If this operation is successful, it proceeds to initialize the `GPT2Model` with the specified configuration. If `gpt2_build_from_checkpoint()` fails, the method returns an error, and the `main` function handles this error by printing an error message and exiting.

This approach encapsulates the initialization logic within the `GPT2Model` struct, making the code more organized and easier to maintain.
```

Q: what does impl mean when used to define a struct?

Q: what does ()? mean when calling a function?

This looks much better:
```rust
struct GPT2Model {
    config: GPT2Config,
    
    // other fields
}

impl GPT2Model {
    fn new() -> io::Result<Self>{
        // Call gpt2_build_from_checkpoint() to load the model from a checkpoint
        // as part of the initialization
        gpt2_build_from_checkpoint()?;
           // I'm assuming the ? means this function may fail, or return a result?

        // Initilize the model with default or specific configuration

        Ok(GPT2Model {
            config: GPT2Config {
                max_seq_len: 1024,
                vocab_size: 50257,
                num_layers: 12,
                num_heads: 12,
                channels: 768,
            },
            // other fields
        })
    }
}
```
That defines the struct and initializes it. 

The main function is now:

```rust
fn main() {
    // Create a new GPT2Model object
    // instead of creating it in main, I'll call the initilization function from within the struct

    let model = GPT2Model::new() {
        Ok(model) => model,
        Err(e) => {
            eprintln!("Failed to initialize GPT2Model: {}", e);
            return;
        }
    };
     
    // Access the fields of the model object
    println!("Model config - max sequence length: {}", model.config.max_seq_len);
    println!("Model config - vocab size: {}", model.config.vocab_size);
    println!("Model config - number of layers: {}", model.config.num_layers);
    println!("Model config - number of heads: {}", model.config.num_heads);
    println!("Model config - number of channels: {}", model.config.channels);
    
}
```
New here is the happy path ```Ok``` and the error path ```Err```.


Does it compile? 

Nope!

```bash
cargo build
   Compiling shouldersofgiants v0.1.0 (/home/efm/git/myrepos/shouldersOfGiants.rs)
error: expected one of `.`, `;`, `?`, `else`, or an operator, found `{`
  --> src/main.rs:59:34
   |
59 |     let model = GPT2Model::new() {
   |                                  ^ expected one of `.`, `;`, `?`, `else`, or an operator

error: could not compile `shouldersofgiants` (bin "shouldersofgiants") due to 1 previous error
```

That looks like a typo.

Yeah, I forgot the match keyword, which I believe is what signals there will be a happy path and an error path.

Once more compile.

And it works!

```bash
cargo build
   Compiling shouldersofgiants v0.1.0 (/home/efm/git/myrepos/shouldersOfGiants.rs)
    Finished dev [unoptimized + debuginfo] target(s) in 0.68s
```

```bash
cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/shouldersofgiants`
The bytes: [198, 215, 52, 1, 1, 0, 0, 0, 0, 4]
Model config - max sequence length: 1024
Model config - vocab size: 50257
Model config - number of layers: 12
Model config - number of heads: 12
Model config - number of channels: 768
```

The line ```The bytes: [198, 215, 52, 1, 1, 0, 0, 0, 0, 4]``` is from the gpt2_build_from_checkpoint function.

## Day 5 2024-04-15 after 968e2c6

Figured out how to print the header of the checkpoint file. 

I needed to set both the initial read of the header and the output to mutable.

```bash
cargo run
   Compiling shouldersofgiants v0.1.0 (/home/efm/git/myrepos/shouldersOfGiants.rs)
    Finished dev [unoptimized + debuginfo] target(s) in 0.61s
     Running `target/debug/shouldersofgiants`
The header bytes: [198, 215, 52, 1, 1, 0, 0, 0, 0, 4, 0, 0, 81, 196, 0, 0, 12, 0, 0, 0, 12, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
Model config - max sequence length: 1024
Model config - vocab size: 50257
Model config - number of layers: 12
Model config - number of heads: 12
Model config - number of channels: 768
```

Next in llm.c are a couple of validity checks on the model header of the checkpoint file -- just making sure a couple of magic numbers are correct.

## Day 6 2024-04-16 
Got the the magic number checks working. 

The file read in Rust defaults to bytes, but the magic numbers are 4 byte (u32). So I had to convert the bytes to the longer size, and then do the integer comparison.

```rust
    let magic_number = u32::from_be_bytes([model_header[0],model_header[1],model_header[2],model_header[3]]);

    if magic_number != 20240327 {
```


[1 Illustrated GPT2](https://jalammar.github.io/illustrated-gpt2/)

[2 tiktoken](https://github.com/openai/tiktoken)

[3 std IO](https://doc.rust-lang.org/std/io/index.html) 
