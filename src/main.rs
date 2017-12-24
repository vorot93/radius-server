#![feature(proc_macro, conservative_impl_trait, generators)]

extern crate futures_await as futures;
extern crate radius_parser as rp;
extern crate semicircle;
extern crate tokio_timer;

use futures::prelude::*;
use tokio_timer::Timer;
use std::time::Duration;

#[async(boxed)]
fn server_handler(
    pkt: semicircle::RadiusMessage,
) -> std::io::Result<Vec<semicircle::RadiusMessage>> {
    println!("Received message from {}:\n{:?}", pkt.addr, pkt.data);

    // We will just sleep here for now. All external I/O and decision making code is up to you.
    await!(Timer::default().sleep(Duration::from_millis(1000))).unwrap();

    let response = vec![semicircle::RadiusMessage {
        addr: pkt.addr,
        data: semicircle::pkt::RadiusData {
            code: rp::RadiusCode::AccessAccept,
            identifier: pkt.data.identifier,
            authenticator: pkt.data.authenticator,
            attributes: vec![],
        },
    }];

    // And here we just return packets that will be sent in return
    Ok(response)
}

fn main() {
    let mut srv = semicircle::Server::new("127.0.0.1:1812".parse().unwrap()).unwrap();

    println!("Setting handler");
    srv.set_handler(server_handler);

    println!("Starting server");
    srv.serve().unwrap();
}
