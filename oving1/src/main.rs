use std::{
    io::{self, Write},
    process::exit,
    sync::{Arc, Mutex},
    thread,
};

// Finn alle primtall mellom to heltall, med et gitt antall tråder

pub fn check_prime(number: usize) -> bool {
    if number % 2 == 0 || number < 3 {
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
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut buf = String::new();

    print!("From: ");
    stdout.flush().expect("Burde kunnet flushe stdout");
    stdin.read_line(&mut buf)?;
    let from: usize = match buf.trim().parse() {
        Ok(from) => {
            buf.clear();
            from
        }
        Err(_) => {
            println!("Det der ser ikke ut til å være et positivt heltall");
            exit(1);
        }
    };

    print!("To: ");
    stdout.flush().expect("Burde kunnet flushe stdout");
    stdin.read_line(&mut buf)?;
    let to: usize = match buf.trim().parse() {
        Ok(to) => {
            buf.clear();
            to
        }
        Err(_) => {
            println!("Det der ser ikke ut til å være et positivt heltall");
            exit(1);
        }
    };

    print!("# of threads: ");
    stdout.flush().expect("Burde kunnet flushe stdout");
    stdin.read_line(&mut buf)?;
    let thread_count: usize = match buf.trim().parse() {
        Ok(thread_count) => {
            buf.clear();
            thread_count
        }
        Err(_) => {
            println!("Det der ser ikke ut til å være et positivt heltall");
            exit(1);
        }
    };

    let mut thread = 0;
    let mut pools: Vec<Vec<usize>> = vec![vec![]; thread_count];
    let mut threads = Vec::new();
    let primes: Arc<Mutex<Vec<usize>>> = Arc::new(Mutex::new(Vec::new()));

    for number in from..=to {
        pools[thread].push(number);
        thread += 1;
        if thread == thread_count {
            thread = 0;
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
        _ = thread.join();
    }

    if let Ok(mut primes) = primes.lock() {
        primes.sort();
        for prime in primes.iter() {
            print!("{prime}");
            if prime != primes.iter().last().unwrap() {
                print!(", ");
            }
        }
        println!();
    }

    Ok(())
}
