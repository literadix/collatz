extern crate crossbeam;

use std::env;

pub struct Collatz {
    curr: u128,
    done: bool,
}

impl Collatz {
    pub fn new(start: u128) -> Collatz {
        Collatz {
            curr: start,
            done: false,
        }
    }
}

impl Iterator for Collatz {
    type Item = u128;

    fn next(&mut self) -> Option<u128> {
        if self.done { return None; };
        let result = Some(self.curr);
        self.curr = match self.curr {
            n if n <= 1 => {
                self.done = true;
                1
            }
            n if n % 2 == 0 => { n / 2 }
            n => { n * 3 + 1 }
        };

        result
    }
}

fn help() {
    println!("usage: collatz <highest number>");
}

#[derive(Debug)]
struct Result {
    start: u128,
    len: usize,
    index_max: usize,
    max: usize,
}

fn calc_slice(slice: &mut [Result]) {
    for (_, e) in slice.iter_mut().enumerate() {
        let collatz: Vec<u128> = Collatz::new(e.start as u128).into_iter().collect();
        let max_value = collatz.iter().fold(e.start, |max, &val| if val > max { val } else { max });
        let index_of_max = collatz.iter().position(|&r| r == max_value).unwrap();

        *e = Result { start: e.start, len: collatz.len(), index_max: index_of_max as usize, max: max_value as usize };
    }
}

fn calc(upper_limit: u128) -> Vec<Result> {
    let num_cpus = num_cpus::get();
    let chunk_size: usize = (upper_limit / num_cpus as u128) as usize;

    let mut nums: Vec<u128> = (0..upper_limit).collect();
    let mut table: Vec<Result> = nums.iter_mut().map(|i| Result { start: *i, len: 0, index_max: 0, max: 0 }).collect();

    let _ = crossbeam::scope(|scope| {
        // Chop `table` into disjoint sub-slices.
        for slice in table.chunks_mut(chunk_size) {
            // Spawn a thread operating on that subslice.
            scope.spawn(move |_| calc_slice(slice));
        }
    });

    table
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            println!("{:?}", args);
            match args[1].parse::<u128>() {
                Ok(upper_limit) => {
                    println!("Calculating ...");
                    for record in calc(upper_limit +1 ) {
                        println!("{:?}", record);
                    }
                }
                _ => help()
            }
        }
        _ => {
            help();
        }
    }
}
