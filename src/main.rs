struct GPT2Config {
    max_seq_len: usize, //max sequence length, e.g. 1024
    vocab_size: usize,  //vocab size, e.g. 50257
    num_layers: usize,  //number of layers, e.g. 12
    num_heads: usize,   //number of heads in attention, e.g. 12
    channels: usize,    //number of channels in fully connected layers, e.g. 768
}

struct GPT2Model {
    config: GPT2Config,
    // other fields
}

fn main() {
    // Create a new GPT2Config object
    let config = GPT2Config {
        max_seq_len: 1024,
        vocab_size: 50257,
        num_layers: 12,
        num_heads: 12,
        channels: 768,
    };

    let model = GPT2Model {
        config: &config,
        // other fields
    };

    // Access the fields of the config object
    println!("Max sequence length: {}", config.max_seq_len);
    println!("Vocab size: {}", config.vocab_size);
    println!("Number of layers: {}", config.num_layers);
    println!("Number of heads: {}", config.num_heads);
    println!("Number of channels: {}", config.channels);

    // Access the fields of the model object
    println!("Model config - max sequence length: {}", model.config.max_seq_len);
    println!("Model config - vocab size: {}", model.config.vocab_size);
    println!("Model config - number of layers: {}", model.config.num_layers);
    println!("Model config - number of heads: {}", model.config.num_heads);
    println!("Model config - number of channels: {}", model.config.channels);

}
