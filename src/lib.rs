#![crate_id = "responsetime"]
#![license = "MIT"]

//! Response time logging ingot for Iron

extern crate iron;
extern crate time;

use iron::{Ingot, Alloy, Request, Response};
use iron::ingot::{Status, Continue};

use time::precise_time_ns;

#[deriving(Clone)]
pub struct ResponseTime {
    entry_time: u64
}

impl ResponseTime {
    pub fn new() -> ResponseTime {
        ResponseTime { entry_time: 0u64 }
    }
}

impl<Rq: Request, Rs: Response> Ingot<Rq, Rs> for ResponseTime {
    fn enter(&mut self, _req: &mut Rq, _res: &mut Rs, _alloy: &mut Alloy) -> Status {
        self.entry_time = precise_time_ns();
        Continue
    }
    fn exit(&mut self, _req: &mut Rq, _res: &mut Rs, _al: &mut Alloy) -> Status {
        let delta = precise_time_ns() - self.entry_time;
        println!("Response time: {} ms", (delta as f64) / 1000000.0);
        Continue
    }
}
