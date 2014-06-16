extern crate iron;
extern crate responsetime;

use std::io::net::ip::Ipv4Addr;

use iron::{Iron, ServerT};

use responsetime::ResponseTime;

fn main() {
    let mut server: ServerT = Iron::new();
    server.smelt(ResponseTime::new());
    server.listen(Ipv4Addr(127, 0, 0, 1), 3000);
}
