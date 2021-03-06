
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

enum OptionEnum {
    FuckError(String)
}
fn main(){
    use bytes::BytesMut;
    use bytes::BufMut;

    let mut a = BytesMut::from(&b"hello"[..]);
    // let mut b = a.split_to(3);
    let mut b = {
        if false {
            Ok(a.split_to(3))
        }else {
            // if true{
            //     return Err("enjie".to_owned());
            // } 
            Err("error".to_owned())
        }
    };
    // b.fuck();
    let d;
    d = a.split_to(4);
    // let c = return_error();
    // c.fuck();
    // println!("{}", char::from((*b)[0]));
    // println!("{}", ((*b)[1] as char));
    // println!("{}", ((*b)[2] as char));
    // println!("{}",a.len());
    // assert_eq!(3, a.len());
    // let 
}

fn return_error() -> OptionEnum{
    return OptionEnum::FuckError("enjie".to_owned());
}