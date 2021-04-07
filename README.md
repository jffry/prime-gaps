This repo is some Rust code for exploring the occurrence of gaps between the primes, inspired by Matt Parker's recent YouTube video at https://www.youtube.com/watch?v=SMsTXQYgbiQ

150 million primes felt a bit small so I grabbed a Rust crate (https://crates.io/crates/primal) which seems like it can efficiently enumerate all the primes in sequence.

As a sanity check, I wanted to make sure the histogram would not grow too huge. Per https://arxiv.org/abs/1309.4053v1, the largest gap between consecutive primes below 10^18 is 1442. There will be no difficulty storing this histogram in memory.

You can generate a CSV with this command (on Windows, use PowerShell and it will work):

```console
cargo run --release | tee hist.csv
```
