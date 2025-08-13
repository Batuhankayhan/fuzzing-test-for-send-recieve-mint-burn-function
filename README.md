# Token Contract Fuzz Test

Rust fuzz testing for a simple token contract using [`proptest`](https://docs.rs/proptest).  
Tests **mint**, **send**, **receive**, and **burn** with randomized inputs.

## Run
```bash
cargo test
# or only the fuzz test:
cargo test fuzz_mint_send_burn -- --nocapture
