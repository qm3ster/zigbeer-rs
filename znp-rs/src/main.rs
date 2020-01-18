#![warn(clippy::all)]

mod areq;
mod serde_znp;
mod sreq;
mod znp_codec;

mod init_coord;

mod cmd;
mod zcl;

mod znp;

use futures_util::StreamExt;

use cmd::types::ShortAddr;

#[tokio::main]
async fn main() {
    let (znp, rec) = znp::Sender::from_path("/dev/ttyACM0");
    let znp = std::sync::Arc::new(futures_util::lock::Mutex::new(znp));
    let znp2 = znp.clone();
    let (close_tx, mut close_rx) = tokio::sync::mpsc::channel::<()>(1);
    tokio::spawn(async {
        let mut rec = rec;
        let znp = znp2;
        let _close_tx = close_tx;
        while let Some(areq) = rec.next().await {
            println!("AREQ: {:x?}", &areq);
            if let Some(sender) = areq.sender() {
                println!("Sender: {:x?}", sender);
            }
            match areq {
                cmd::Areq::Af(cmd::af::In::IncomingMsg(incoming)) => {
                    let frame = zcl::frame::ZclFrame::parse(bytes::Bytes::from(incoming.data));
                    println!("ZclFrame: {:x?}", frame);
                    let cluster = zcl::clusters::ClusterId::from(incoming.cluster);
                    println!("Cluster: {:x?}", cluster);
                    if let Ok(cluster) = cluster {
                        let msg = zcl::clusters::In::parse(cluster, frame);
                        println!("{:x?}", msg);
                    }
                }
                cmd::Areq::Zdo(cmd::zdo::In::EndDevAnnce(announcement)) => {
                    // tokio::timer::delay_for(std::time::Duration::from_millis(100)).await;
                    let mut znp = znp.lock().await;
                    interrogate(&mut znp, announcement.nwk_addr).await;
                }
                _ => {}
            };
        }
    });

    {
        let mut znp = znp.lock().await;
        init_coord::init(&mut znp).await;
    }
    close_rx.next().await;

    // use cmd::sys::StartTimer;
    // for timer_id in 0..=3 {
    //     let cmd = StartTimer {
    //         timer_id,
    //         timeout: 50 - 10 * timer_id as u16,
    //     };
    //     let res = znp.sreq(cmd).await;
    //     println!("StartTimer {:x?}", res);
    // }

    // init_coord::soft_reset(&mut znp).await;

    // blink_forever(&mut znp).await;
}

async fn interrogate(znp: &mut znp::Sender, device: ShortAddr) {
    use cmd::types::Endpoint;

    {
        let cmd = cmd::zdo::PowerDescReq {
            dest_addr: device,
            query_addr: device,
        };
        let res = znp.sreq(cmd).await;
        println!("PowerDescReq {:x?}", res);
    }

    {
        let cmd = cmd::zdo::ActiveEpReq {
            dest_addr: device,
            query_addr: device,
        };
        let res = znp.sreq(cmd).await;
        println!("ActiveEpReq {:x?}", res);
    }

    {
        let cmd = cmd::zdo::SimpleDescReq {
            dest_addr: device,
            query_addr: device,
            endpoint: Endpoint(1),
        };
        let res = znp.sreq(cmd).await;
        println!("SimpleDescReq {:x?}", res);
    }

    {
        let cmd = cmd::zdo::ComplexDescReq {
            dest_addr: device,
            query_addr: device,
        };
        let res = znp.sreq(cmd).await;
        println!("ComplexDescReq {:x?}", res);
    }
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
        tokio::time::delay_for(Duration::from_millis(if on { 1 } else { 4000 })).await;
    }
}
