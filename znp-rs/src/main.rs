#![feature(await_macro, async_await, futures_api)]

#[macro_use]
extern crate tokio;

use tokio::prelude::*;

mod areq;
mod serde_znp;
mod sreq;
mod znp_codec;

mod init_coord;

mod cmd;

mod znp;
fn main() {
    tokio::run_async(
        async {
            let (mut znp, rec) = znp::Sender::from_path("/dev/ttyACM0");
            tokio::spawn_async(
                async {
                    let mut rec = rec;
                    while let Some(areq) = await!(rec.next()) {
                        println!("AREQ: {:?}", areq);
                    }
                },
            );
            use cmd::zb::{ZbDeviceInfoProp, ZbGetDeviceInfoReq};
            for param in vec![
                ZbDeviceInfoProp::DevState,
                ZbDeviceInfoProp::IeeeAddr,
                ZbDeviceInfoProp::ShortAddr,
            ] {
                let cmd = ZbGetDeviceInfoReq { param };
                let res = await!(znp.sreq(cmd));
                println!("{:x?}", res);
            }

            use cmd::sys::NvRead;
            let cmd = NvRead {
                /// ZNP_HAS_CONFIGURED
                id: 0x0F00,
                offset: 0x00,
            };
            let res = await!(znp.sreq(cmd));
            // Expecting [0x55]
            println!("{:x?}", res);

            await!(init_coord::init(&mut znp));

            use cmd::zdo::StartupFromApp;
            let cmd = StartupFromApp {
                delay: 100, /* this was 100, why? When would you want this? */
            };
            let res = await!(znp.sreq(cmd));
            println!("StartupFromApp {:x?}", res);

            use cmd::sys::StartTimer;
            for timer_id in 0..=3 {
                let cmd = StartTimer {
                    timer_id,
                    timeout: 50 - 10 * timer_id as u16,
                };
                let res = await!(znp.sreq(cmd));
                println!("StartTimer {:x?}", res);
            }

            await!(init_coord::soft_reset(&mut znp));

            await!(blink_forever(&mut znp));
        },
    );
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
