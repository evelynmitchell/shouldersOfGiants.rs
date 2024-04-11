
use std::io;
use std::io::prelude::*;
use std::fs::File;



struct GPT2Config {
    max_seq_len: usize, //max sequence length, e.g. 1024
    vocab_size: usize,  //vocab size, e.g. 50257
    num_layers: usize,  //number of layers, e.g. 12
    num_heads: usize,   //number of heads in attention, e.g. 12
    channels: usize,    //number of channels in fully connected layers, e.g. 768
} // usize is an unsigned integer type, the exact size depends on the platform

struct GPT2Model {
    config: GPT2Config,
    gpt2_build_from_checkpoint: fn() -> io::Result<()>,
    // other fields
}

fn gpt2_build_from_checkpoint() -> io::Result<()> {
    let mut f = File::open("gpt2_124M.bin")?;
    let mut buffer = [0; 10];

    // read up to 10 bytes
    let n = f.read(&mut buffer)?;

    println!("The bytes: {:?}", &buffer[..n]);
    Ok(())
}

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
        gpt2_build_from_checkpoint:gpt2_build_from_checkpoint(),
        // other fields
    };

    // Access the fields of the model object
    println!("Model config - max sequence length: {}", model.config.max_seq_len);
    println!("Model config - vocab size: {}", model.config.vocab_size);
    println!("Model config - number of layers: {}", model.config.num_layers);
    println!("Model config - number of heads: {}", model.config.num_heads);
    println!("Model config - number of channels: {}", model.config.channels);
    
}
