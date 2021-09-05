This program uses the Collatz sequences to demonstrate the 
parallel calculation of tasks in Rust. We use the crossbeam library
for the parallelization.

Test this code with gitpod.io:

[Open project for free in gipod.io](https://gitpod.io/#https://github.com/literadix/collatz)

There

    cargo build --release
    ./target/release/collatz 100 
    time ./target/release/collatz 1000000 > /dev/null