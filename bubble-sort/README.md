# Rust Bubblesort
```
$ cargo run
   Compiling rust-bubblesort v0.1.0 (/Users/shunjizhan/Acala/Rust-bubblesort)
    Finished dev [unoptimized + debuginfo] target(s) in 0.28s
     Running `target/debug/rust-bubblesort`
before sort:[5, 2, 9, 1, 5, 6]
after sort:[1, 2, 5, 5, 6, 9]
before sort:["apple", "orange", "banana", "pear"]
after sort:["apple", "banana", "orange", "pear"]
```

```
$ cargo test
   Compiling rust-bubblesort v0.1.0 (/Users/shunjizhan/Acala/Rust-bubblesort)
    Finished test [unoptimized + debuginfo] target(s) in 0.33s
     Running unittests src/main.rs (target/debug/deps/rust_bubblesort-f728d96d643d6802)

running 2 tests
test test_bubble_sort_string ... ok
test test_bubble_sort_i32 ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```