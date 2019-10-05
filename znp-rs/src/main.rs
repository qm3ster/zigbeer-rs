#![feature(await_macro, async_await, futures_api)]
#![warn(bare_trait_objects)]

use tokio::prelude::*;

mod areq;
mod serde_znp;
mod sreq;
mod znp_codec;

mod init_coord;

mod cmd;
mod zcl;

mod znp;


    use futures::{FutureExt, TryFutureExt};
    use futures::compat::Future01CompatExt;


fn main() {
    let app = async {
        let (mut znp, rec) = znp::Sender::from_path("/dev/ttyACM0");
        tokio::spawn(async {
            let mut rec = rec;
            rec.for_each(|areq| {
                println!("AREQ: {:x?}", &areq);
                if let cmd::Areq::Af(cmd::af::In::IncomingMsg(incoming)) = areq {
                    let msg = zcl::clusters::In::from_incoming(&incoming);
                    println!("{:?}", msg);
                }
                Ok(())
            }).compat().await;
        }.unit_error().boxed().compat());

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
    };
                    tokio::run(app.unit_error().boxed().compat());

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
        ).compat())
        .unwrap();
    }
}
