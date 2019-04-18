#![feature(await_macro, async_await, futures_api)]
#![warn(bare_trait_objects)]

#[macro_use]
extern crate tokio;

use tokio::prelude::*;

mod areq;
mod serde_znp;
mod sreq;
mod znp_codec;

mod init_coord;

mod cmd;
mod zcl;

mod znp;
fn main() {
    tokio::run_async(async {
        let (mut znp, rec) = znp::Sender::from_path("/dev/ttyACM0");
        tokio::spawn_async(async {
            let mut rec = rec;
            while let Some(areq) = await!(rec.next()) {
                println!("AREQ: {:x?}", &areq);
                if let Ok(cmd::Areq::Af(cmd::af::In::IncomingMsg(msg))) = areq {
                    use bytes::buf::IntoBuf;
                    let msg = crate::zcl::frame::ZclFrame::parse(msg.data.into_buf());
                    println!("{:?}", msg);
                }
            }
        });

        await!(init_coord::init(&mut znp));

        use cmd::sys::StartTimer;
        for timer_id in 0..=3 {
            let cmd = StartTimer {
                timer_id,
                timeout: 50 - 10 * timer_id as u16,
            };
            let res = await!(znp.sreq(cmd));
            println!("StartTimer {:x?}", res);
        }

        // await!(init_coord::soft_reset(&mut znp));

        await!(blink_forever(&mut znp));
    });
}

async fn blink_forever(znp: &mut znp::Sender) {
    use cmd::util::{UtilLedControl, UtilLedControlRsp};
    for id in 1..=2 {
        let cmd = UtilLedControl {
            led_id: id,
            mode: false,
        };
        let res = await!(znp.sreq(cmd));
        println!("Light Off {:x?}", res);
    }

    let led_id = 2;
    let mut on = true;
    loop {
        on = !on;
        let cmd = UtilLedControl { led_id, mode: on };
        let res = await!(znp.sreq(cmd));
        match res {
            Ok(UtilLedControlRsp { status: 0 }) => {}
            _ => println!("Couldn't toggle light: {:?}", res),
        }
        use std::time::{Duration, Instant};
        await!(tokio::timer::Delay::new(
            Instant::now() + Duration::from_millis(if on { 1 } else { 4000 })
        ))
        .unwrap();
    }
}
