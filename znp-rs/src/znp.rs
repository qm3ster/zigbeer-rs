use super::sreq::Sreq;
use super::znp_codec;
use futures::lock::Mutex;
use std::path::Path;
use tokio::prelude::*;
use tokio::sync::{mpsc, oneshot};
use tokio_serial::{Serial, SerialPortSettings};
use znp_codec::{ZnpCodec, ZpiCmd};
pub struct Znp {
    tx: Mutex<Option<tokio::prelude::stream::SplitSink<tokio::codec::Framed<Serial, ZnpCodec>>>>,
    cbs: mpsc::Sender<oneshot::Sender<ZpiCmd>>,
}
impl Znp {
    pub fn from_path<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        let sp_settings = SerialPortSettings {
            baud_rate: 115_000,
            ..Default::default() // 8-N-1 is default
        };
        let sp = Serial::from_path(path, &sp_settings).unwrap();
        let sp = tokio::codec::Framed::new(sp, ZnpCodec);
        type Callback = oneshot::Sender<ZpiCmd>;
        let (ctx, cbs_rx) = mpsc::channel::<Callback>(1);
        let (stx, sp_rx) = sp.split();
        tokio::spawn_async(
            async {
                let mut cbs_rx = cbs_rx;
                let mut sp_rx = sp_rx;
                while let Some(frame) = await!(sp_rx.next()) {
                    let frame = frame.unwrap();
                    use znp_codec::Type::{AREQ, SRSP};
                    match frame.typ() {
                        SRSP => {
                            if let Ok(Async::Ready(Some(cb))) = cbs_rx.poll() {
                                cb.send(frame).unwrap();
                            } else {
                                eprintln!("Unexpected SRSP: {:?}", frame);
                                panic!("SRSP no one was waiting for");
                            }
                        }
                        AREQ => {
                            use crate::cmd::Areq;
                            println!("AREQ:{:?}", Areq::from_subsys(frame));
                        }
                        _ => panic!("incoming POLL or SREQ"),
                    }
                }
            },
        );
        Znp {
            tx: Mutex::new(Some(stx)),
            cbs: ctx,
        }
    }
    pub async fn sreq<S>(&mut self, req: S) -> S::Srsp
    where
        S: Sreq + 'static,
    {
        let mut tx_lock = await!(self.tx.lock());
        let send = tx_lock.take().unwrap().send(req.frame());
        let (cb_tx, cb_rx) = oneshot::channel();
        await!(self.cbs.clone().send(cb_tx)).unwrap();
        let sp_tx = await!(send).unwrap();
        let srsp = await!(cb_rx).unwrap();
        let srsp = S::parse_res(srsp);
        *tx_lock = Some(sp_tx);
        srsp
    }
}
