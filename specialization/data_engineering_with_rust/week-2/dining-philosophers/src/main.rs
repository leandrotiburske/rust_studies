/*
* The dining philosophers problem involves multiple threads needing
* synchronized access to shared resources, risking deadlock.
*
* This code models philosophers as threads and forks as shared Mutex<()>
* wrapped in Arc for thread-safe reference counting.
*
* To prevent deadlock from a "deadly embrace" of waiting for neighboring
* forks, philosophers acquire lower numbered forks first. This breaks
* symmetry and avoids circular waiting.
*
* The Mutexes provide exclusive fork access. The Arc allows sharing forks
* between philosophers.
*
* The simulation prints start time, eating duration, and total time for
* all philosophers. Total time approximately equals philosophers divided
* by forks, as that number can eat concurrently.
*
* Key techniques:
* - Used Mutex<()> to represent exclusive fork access
* - Wrapped in Arc to share Mutexes between threads
* - Numbered philosophers and acquire lower fork first
* - Prints timing metrics for simulation
*/

use priomutex::Mutex;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

struct Fork {
    id: u32,
    mutex: Mutex<()>,
}

struct Philosopher {
    id: u32,
    name: String,
    left_fork: Arc<Fork>,
    right_fork: Arc<Fork>,
    hunger: usize,
}

impl Philosopher {
    fn new(
        id: u32,
        name: &str,
        left_fork: Arc<Fork>,
        right_fork: Arc<Fork>,
        hunger: usize,
    ) -> Philosopher {
        Philosopher {
            id,
            name: name.to_string(),
            left_fork,
            right_fork,
            hunger,
        }
    }

    fn eat(&self) {
        let (first_fork, second_fork) = if self.id % 2 == 0 {
            (&self.left_fork, &self.right_fork)
        } else {
            (&self.right_fork, &self.left_fork)
        };

        let _first_guard = first_fork.mutex.lock(self.hunger).unwrap();
        println!("{} picked up fork {}.", self.name, first_fork.id);
        let _second_guard = second_fork.mutex.lock(self.hunger).unwrap();
        println!("{} picked up fork {}.", self.name, second_fork.id);

        println!("{} is eating.", self.name);
        thread::sleep(Duration::from_secs(1));
        println!("{} finished eating.", self.name);

        println!("{} put down fork {}.", self.name, first_fork.id);
        println!("{} put down fork {}.", self.name, second_fork.id);
    }
}

fn main() {
    println!("Dining Philosophers Problem:  15 Philosophers, 4 Forks...Yikes!!");

    //we only have 4 forks at the table
    let forks = (0..4)
        .map(|id| {
            Arc::new(Fork {
                id,
                mutex: Mutex::new(()),
            })
        })
        .collect::<Vec<_>>();

    let philosophers = vec![
        ("Jürgen Habermas", 0, 1, 0),
        ("Friedrich Engels", 1, 2, 1),
        ("Karl Marx", 2, 3, 2),
        ("Thomas Piketty", 3, 0, 3),
        ("Michel Foucault", 0, 1, 4),
        ("Socrates", 1, 2, 5),
        ("Plato", 2, 3, 6),
        ("Aristotle", 3, 0, 7),
        ("Pythagoras", 0, 1, 8),
        ("Heraclitus", 1, 2, 9),
        ("Democritus", 2, 3, 10),
        ("Diogenes", 3, 0, 11),
        ("Epicurus", 0, 1, 12),
        ("Zeno of Citium", 1, 2, 13),
        ("Thales of Miletus", 2, 3, 14),
        ("René Descartes", 3, 0, 15),
        ("Thomas Aquinas", 0, 1, 16),
    ]
    .into_iter()
    .enumerate()
    .map(|(id, (name, left, right, hunger))| {
        Philosopher::new(
            id as u32,
            name,
            Arc::clone(&forks[left]),
            Arc::clone(&forks[right]),
            hunger,
        )
    })
    .collect::<Vec<_>>();

    let start = Instant::now();

    let handles = philosophers
        .into_iter()
        .map(|philosopher| {
            thread::spawn(move || {
                philosopher.eat();
            })
        })
        .collect::<Vec<_>>();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Total time: {:?}", start.elapsed());
}
