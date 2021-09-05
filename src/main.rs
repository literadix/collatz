use std::env;
use std::thread;

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

fn calc(upper_limit: u128) -> Vec<Result> {
    let num_cpus = num_cpus::get();
    let numbers: Vec<u128> = (1..upper_limit).collect();
    let numbers_chunks: Vec<&[u128]> = numbers.chunks(num_cpus).collect();

    let mut results: Vec<Result> = Vec::new();
    for chunk in numbers_chunks {
        for s in chunk {
            let collatz: Vec<u128> = Collatz::new(*s).into_iter().collect();
            let max_value = collatz.iter().fold(0u128, |max, &val| if val > max { val } else { max });
            let index_of_max = collatz.iter().position(|&r| r == max_value).unwrap();
            //println! ("{:?} [{},{:?}[{}]]: {:?} ", s, collatz.len(), max_value, index_of_max ,collatz);

            results.push(Result {
                start: *s,
                len: collatz.len(),
                index_max: max_value as usize,
                max: index_of_max,
            });
        }
    }

    // https://doc.rust-lang.org/rust-by-example/std_misc/threads/testcase_mapreduce.html
    // https://stackoverflow.com/questions/33818141/how-do-i-pass-disjoint-slices-from-a-vector-to-different-threads
    // https://crates.io/crates/crossbeam
    let mut thread_children = vec![];
    for i in 0..num_cpus {

        // Spin up another thread
        thread_children.push(thread::spawn(move || -> Vec<Result>{
            println!("Thread {:?}", i);
            let results: Vec<Result> = Vec::new();
            results
        }));
    }

    for child in thread_children {
        // Wait for the thread to finish. Returns a result.
        let results = child.join();
        for result in results.unwrap() {
            println!("Thread {:?}", result);
        }
    }

    results
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            println!("{:?}", args);
            match args[1].parse() {
                Ok(upper_limit) => {
                    println!("Calculating ...");

                    #[cfg(not(debug_assertions))]
                    println!("{:?}", calc(upper_limit).len());

                    #[cfg(debug_assertions)]
                    for record in calc(upper_limit) {
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
