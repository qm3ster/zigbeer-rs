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
        let (ctx, crx) = mpsc::channel::<Callback>(1);
        let (stx, srx) = sp.split();
        tokio::spawn_async(
            async {
                let mut crx = crx;
                let mut srx = srx;
                let mut cb: Option<Callback> = None;
                while let Some(frame) = await!(srx.next()) {
                    if cb.is_none() {
                        if let Ok(Async::Ready(new_cb)) = crx.poll() {
                            cb = new_cb;
                        }
                    }
                    if cb.is_some() {
                        cb.take().unwrap().send(frame.unwrap()).unwrap();
                    } else {
                        // println!("{:?}", &frame);
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
        let (crx, ctx) = oneshot::channel();
        await!(self.cbs.clone().send(crx)).unwrap();
        let tx = await!(send).unwrap();
        let srsp = await!(ctx).unwrap();
        let srsp = S::parse_res(srsp);
        *tx_lock = Some(tx);
        srsp
    }
}
