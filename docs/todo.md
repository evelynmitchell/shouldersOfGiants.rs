# TODOS

This is a place to put things to do outside of the main README log.

## code related

[ ] add a configuration loader from the environment or filesystem for loading the model config

## learning related

[ ] spend a bit more time with the Rust book (https://rust-book.cs.brown.edu/experiment-intro.html) 

## planning related

[ ] data structures: ParameterTensors
[ ] data structures: ActivationTensors
[ ] data structures: GPT2
[ ] rewrite gpt2_build_from_checkpoint to load the checkpoint, instead of just printing the first 10 bytes
[ ] Investigate how to incorporate a CUDA kernel into Rust
[ ] would it make sense to have a Rust crate of CUDA kernels?
[ ] add gh cicd to run the build and tests

## completed
[x] file loading: checkpoints
[x] file loading: where is gpt2_124M.bin ?
    This file is created by the reference python implementation train_gpt2.py
[x] create the model checkpoint gpt2_124M.bin and gpt2_124M_debug_state.bin