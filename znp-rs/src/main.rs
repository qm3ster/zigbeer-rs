#![feature(await_macro, async_await, futures_api)]

#[macro_use]
extern crate tokio;

mod serde_znp;
mod sreq;
mod znp_codec;

mod cmd;
use cmd::util::UtilLedControl;

mod znp;
fn main() {
    tokio::run_async(
        async {
            let mut znp = znp::Znp::from_path("/dev/ttyACM0");
            use cmd::zb::{ZbDeviceInfoProp, ZbGetDeviceInfoReq};
            for param in vec![
                ZbDeviceInfoProp::DevState,
                ZbDeviceInfoProp::IeeeAddr,
                ZbDeviceInfoProp::ShortAddr,
            ] {
                let v = ZbGetDeviceInfoReq { param };
                let res = await!(znp.sreq(v));
                println!("{:x?}", res);
            }

            for id in 1..=2 {
                let v = UtilLedControl {
                    led_id: id,
                    mode: false,
                };
                let res = await!(znp.sreq(v));
                println!("{:x?}", res);
            }

            let id = 1;
            let mut on = true;
            loop {
                for i in (0..1000).chain((0..1000).rev()) {
                    on = !on;
                    let v = UtilLedControl {
                        led_id: id,
                        mode: on,
                    };
                    let _res = await!(znp.sreq(v));
                    // println!("{:x?}", res);
                    use std::time::{Duration, Instant};
                    await!(tokio::timer::Delay::new(
                        Instant::now() + Duration::from_micros(if on { i } else { 1000 - i })
                    ))
                    .unwrap();
                }
            }
        },
    );
}
