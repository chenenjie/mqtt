
extern crate futures;
extern crate futures_cpupool;
extern crate tokio_timer;
extern crate bytes;

use std::time::Duration;

use futures::Future;
use futures_cpupool::CpuPool;
use tokio_timer::Timer;

pub enum Chain<A, B, C> {
    First(A, C),
    Second(B),
    Done,
}

impl<A, B, C> Chain<A, B, C> {
    pub fn new(a: A, c: C) -> Chain<A, B, C> {
        Chain::First(a, c)
    }
}



const BIG_PRIME: u64 = 1;
fn is_prime(num: u64) -> bool { true }

fn test() {
    let pool = CpuPool::new_num_cpus();
    let timer = Timer::default();

    // a future that resolves to Err after a timeout
    let timeout = timer.sleep(Duration::from_millis(750))
        .then(|_| Err(()));

    // a future that resolves to Ok with the primality result
    let prime = pool.spawn_fn(|| {
        Ok(is_prime(BIG_PRIME))
    });

    // a future that resolves to one of the above values -- whichever
    // completes first!
    let winner = timeout.select(prime).map(|(win, _)| win);

    // now block until we have a winner, then print what happened
    match winner.wait() {
        Ok(true) => println!("Prime"),
        Ok(false) => println!("Not prime"),
        Err(_) => println!("Timed out"),
    }
    //不可以推断b的类型
    //let chain = Chain::new(1i32, 2i32);
}

fn main(){
    // let 
}