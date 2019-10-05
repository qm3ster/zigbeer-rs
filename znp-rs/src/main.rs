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

use futures::{future, stream::StreamExt};

#[tokio::main]
async fn main() {
    let (mut znp, rec) = znp::Sender::from_path("/dev/ttyACM0");
    tokio::spawn(async {
        rec.for_each(|areq| {
            println!("AREQ: {:x?}", &areq);
            if let cmd::Areq::Af(cmd::af::In::IncomingMsg(incoming)) = areq {
                let msg = zcl::clusters::In::from_incoming(&incoming);
                println!("{:?}", msg);
            }
            future::ready(())
        })
        .await;
    });

    init_coord::init(&mut znp).await;

    use cmd::sys::StartTimer;
    for timer_id in 0..=3 {
        let cmd = StartTimer {
            timer_id,
            timeout: 50 - 10 * timer_id as u16,
        };
        let res = znp.sreq(cmd).await;
        println!("StartTimer {:x?}", res);
    }

    // init_coord::soft_reset(&mut znp).await;

    blink_forever(&mut znp).await;
}

async fn blink_forever(znp: &mut znp::Sender) {
    use cmd::util::{UtilLedControl, UtilLedControlRsp};
    for id in 1..=2 {
        let cmd = UtilLedControl {
            led_id: id,
            mode: false,
        };
        let res = znp.sreq(cmd).await;
        println!("Light Off {:x?}", res);
    }

    let led_id = 2;
    let mut on = true;
    loop {
        on = !on;
        let cmd = UtilLedControl { led_id, mode: on };
        let res = znp.sreq(cmd).await;
        match res {
            Ok(UtilLedControlRsp { status: 0 }) => {}
            _ => println!("Couldn't toggle light: {:?}", res),
        }
        use std::time::Duration;
        tokio::timer::delay_for(Duration::from_millis(if on { 1 } else { 4000 })).await;
    }
}
