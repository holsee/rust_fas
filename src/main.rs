extern crate futures;
extern crate futures_cpupool;

use futures::Future;
use futures_cpupool::CpuPool;

//const BIG_PRIME: u64 = 15_485_867;
//const BIG_PRIME: u64 = 104_395_303;
const BIG_PRIME: u64 = 982_451_653;

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

    //sync();
    //sync();
    //sync();
    //sync()
}

fn async() {
    let pool = CpuPool::new(4);

    let fprime0 = pool.spawn_fn(|| -> Result<bool, ()> {
        Ok(is_prime(BIG_PRIME))
    });

    let fprime1 = pool.spawn_fn(|| -> Result<bool, ()> {
        Ok(is_prime(BIG_PRIME))
    });

    let fprime2 = pool.spawn_fn(|| -> Result<bool, ()> {
        Ok(is_prime(BIG_PRIME))
    });

    let fprime3 = pool.spawn_fn(|| -> Result<bool, ()> {
        Ok(is_prime(BIG_PRIME))
    });

    let r0 = fprime0.wait().unwrap();
    let r1 = fprime1.wait().unwrap();
    let r2 = fprime2.wait().unwrap();
    let r3 = fprime3.wait().unwrap();

    // wait for computation, unwrap result, print
    println!("Prime? {} {} {} {}", r0, r1, r2, r3)
}

// fn sync() {
//     let result: Result<bool, ()> = Ok(is_prime(BIG_PRIME));
//     println!("Sync Prime? {}", result.unwrap())
// }

