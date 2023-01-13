use std::{
    io::{self, Write},
    sync::{Arc, Mutex},
    thread,
};

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    from: usize,
    #[arg(short, long)]
    to: usize,
    #[arg(short, long)]
    n: usize,
}

// Finn alle primtall mellom to heltall, med et gitt antall tråder

pub fn check_prime(number: usize) -> bool {
    if (number > 2 && number % 2 == 0) || number < 2 {
        false
    } else {
        for factor in (3..=(number as f64).sqrt() as usize).step_by(2) {
            if number % factor == 0 {
                return false;
            }
        }

        true
    }
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let mut thread = 0;
    let mut pools: Vec<Vec<usize>> = vec![vec![]; args.n];
    let mut threads = Vec::new();
    let primes: Arc<Mutex<Vec<usize>>> = Arc::new(Mutex::new(Vec::new()));

    for number in args.from..=args.to {
        // Tar bare med oddetall, siden partall ikke kan være primtall (unntatt 2)
        if number % 2 != 0 || number == 2 {
            pools[thread].push(number);
            thread += 1;

            if thread == args.n {
                thread = 0;
            }
        }
    }

    for pool in pools {
        let primelock = primes.clone();
        threads.push(thread::spawn(move || {
            pool.iter()
                .filter(|&number| check_prime(*number))
                .for_each(|prime| {
                    let mut primes = primelock.lock().unwrap();
                    primes.push(*prime);
                });
        }))
    }

    for thread in threads {
        thread.join().expect("Kunne ikke avslutte tråd som forventet, avbryter");
    }

    if let Ok(mut primes) = primes.lock() {
        primes.sort();
        print!("[ ");
        for prime in primes.iter() {
            print!("{prime} ");
        }
        println!("]");
    }

    Ok(())
}
