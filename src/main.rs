struct GPT2Config {
    max_seq_len: usize, //max sequence length, e.g. 1024
    vocab_size: usize,  //vocab size, e.g. 50257
    num_layers: usize,  //number of layers, e.g. 12
    num_heads: usize,   //number of heads in attention, e.g. 12
    channels: usize,    //number of channels in fully connected layers, e.g. 768
} // usize is an unsigned integer type, the exact size depends on the platform

struct GPT2Model {
    config: GPT2Config,
    // karpathy loads the GPT2 checkpoints from a file
    //  in gpt2_build_from_checkpoint 
    //  Q: Where do I get gpt2_124M.bin?
    // other fields
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
        // other fields
    };

    // Access the fields of the model object
    println!("Model config - max sequence length: {}", model.config.max_seq_len);
    println!("Model config - vocab size: {}", model.config.vocab_size);
    println!("Model config - number of layers: {}", model.config.num_layers);
    println!("Model config - number of heads: {}", model.config.num_heads);
    println!("Model config - number of channels: {}", model.config.channels);
    
}
