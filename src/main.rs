use rand::Rng;
use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const DEFAULT_NUM_PHILOSOPHERS: usize = 5;
const SIMULATION_TIME: u64 = 60 /* seconds */ * 3;

/// A philosopher picks up the specified fork on the table.
fn eat(table: &Arc<Vec<Mutex<()>>>, philosopher: usize, eat_time: u64) {
    let table = table.clone();
    let len = table.len();
    // Here the philosopher will pick up their forks one at a time.
    // If each philosopher picks up the left fork first, a deadlock occur because each philosopher will be
    // waiting for their respective right fork to be available. Since all forks are taken, no progress will
    // be made, hence the deadlock.
    // To avoid this problem, every other philosopher will pick up the left fork first then the right while
    // the rest will pick up the right fork first. This avoids the deadlock as the worst possible scenario
    // only one philosopher will be eating. Since the philosopher won't be eating forever, we will
    // eventually enter a state that allows the neighboring philosophers to pick up their missing fork.

    // This is translated to every even numberd philosopher will pick up their left fork first and every odd
    // numbered philosopher will pick up their right fork first.
    let first_fork = if philosopher % 2 == 0 {
        (philosopher + len) % len
    } else {
        (philosopher + len + 1) % len
    };

    let second_fork = if philosopher % 2 == 0 {
        (philosopher + len + 1) % len
    } else {
        (philosopher + len) % len
    };

    let _first_fork = table[first_fork].lock().unwrap();
    let _second_fork = table[second_fork].lock().unwrap();
    println!(
        "Philosopher {} currently eating for {} seconds",
        philosopher, eat_time
    );
    thread::sleep(Duration::from_secs(eat_time as u64));
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let num_philosophers = args
        .get(1)
        .and_then(|num_str| num_str.parse::<usize>().ok())
        .or_else(|| {
            print!("Invalid/no argument given. ");
            println!("Using default value: {}.", DEFAULT_NUM_PHILOSOPHERS);
            println!();
            Some(DEFAULT_NUM_PHILOSOPHERS)
        })
        .unwrap();

    // The number of forks is always equal to the number of philosophers.
    let mut forks = Vec::<Mutex<()>>::with_capacity(num_philosophers);
    for _ in 0..num_philosophers {
        forks.push(Mutex::new(()));
    }

    let table = Arc::new(forks);
    for phil in 0..num_philosophers {
        let table = table.clone();
        thread::spawn(move || {
            // Create random number generator for this thread.
            let mut rng = rand::thread_rng();
            loop {
                // A philosopher is allowed to think for a maximum of 32 seconds. This is just so we can see results
                // often rather than waiting an absurdly long time. In actuality, it can be any number of seconds.
                let think_time = (rng.gen::<u8>() >> 3) + 1;
                println!(
                    "Philosopher {} thinking for {} seconds...",
                    phil, think_time
                );
                thread::sleep(Duration::from_secs(think_time as u64));

                // Likewise, a philosopher is allowed to eat for a maximum of 32 seconds.
                let eat_time = (rng.gen::<u8>() >> 3) + 1;
                println!("Philosopher {} preparing to eat...", phil);
                eat(&table, phil, eat_time as u64);

                println!("Philosopher {} done eating", phil);
                println!();
                // thread::sleep(Duration::from_millis(1));
            }
        });
    }
    thread::sleep(Duration::from_secs(SIMULATION_TIME));
}
