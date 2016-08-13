
## glomp.edu
#### Build & run
```sh
$ cargo build
$ cargo run
```
#### Run tests
```sh
$ cargo test
```
#### Pseudo-automated tests
hacky tests requiring manual intervention are ignored by default, to run them (note they must be run sequentially):
```sh
$ RUST_TEST_THREADS=1 cargo test -- --ignored --nocapture
```
