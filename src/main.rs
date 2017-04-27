extern crate futures;
extern crate futures_cpupool;
extern crate tokio_timer;

use std::time::Duration;
use tokio_timer::Timer;

use futures::Future;
use futures_cpupool::CpuPool;


//const BIG_PRIME: u64 = 15_485_867;
const BIG_PRIME: u64 = 104_395_303;
//const BIG_PRIME: u64 = 982_451_653;

// checks whether a number is prime, slowly
fn is_prime(num: u64) -> bool {
    for i in 2..num {
        if num % i == 0 {
            println!("{}", i);
            return false
        }
    }
    true
}

fn main() {
    async()
}

fn async() {
    let pool = CpuPool::new(4);
    let timer = Timer::default();

    // a future that resolves to Err after a timeout
    let timeout = timer.sleep(Duration::from_millis(1500))
                       .then(|_| Err(()));

    let fprime = pool.spawn_fn(|| -> Result<bool, ()> {
        Ok(is_prime(BIG_PRIME))
    });

    // a future that resolves to one of the above values -- whichever
    // completes first!
    let winner = timeout.select(fprime)
                        .map(|(win, _)| win);

    // now block until we have a winner, then print what happened
    match winner.wait() {
        Ok(true) => println!("Prime"),
        Ok(false) => println!("Not prime"),
        Err(_) => println!("Timed out"),
    }
}

