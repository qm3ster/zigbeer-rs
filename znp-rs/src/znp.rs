use super::sreq::Sreq;
use super::znp_codec;
use crate::cmd;
use futures::lock::Mutex;
use std::path::Path;
use tokio::prelude::*;
use tokio::sync::{mpsc, oneshot};
use tokio_serial::{Serial, SerialPortSettings};
use znp_codec::{ZnpCodec, ZnpCmd};
pub struct Znp {
    tx: Mutex<Option<tokio::prelude::stream::SplitSink<tokio::codec::Framed<Serial, ZnpCodec>>>>,
    cbs: mpsc::Sender<oneshot::Sender<ZnpCmd>>,
}
#[derive(Debug)]
pub enum SreqError {
    BadResponse(cmd::error::Error),
    SerialPortGone,
    IO(std::io::Error),
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
        type Callback = oneshot::Sender<ZnpCmd>;
        let (ctx, cbs_rx) = mpsc::channel::<Callback>(1);
        let (stx, sp_rx) = sp.split();
        tokio::spawn_async(
            async {
                let mut cbs_rx = cbs_rx;
                let mut sp_rx = sp_rx;
                while let Some(frame) = await!(sp_rx.next()) {
                    use znp_codec::Type::{AREQ, SRSP};
                    match frame {
                        Err(err) => {
                            eprintln!("{}", err);
                            break;
                        }
                        Ok(frame) => match frame.typ() {
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
                                match Areq::from_subsys(frame) {
                                    Ok(areq) => println!("Known AREQ: {:?}", areq),
                                    Err(cmd::error::Error::Unimplemented { subsys, cmd_id }) => {
                                        println!(
                                            "Unimplemented AREQ: {:?} Cmd1 = {:#X?}",
                                            subsys, cmd_id
                                        )
                                    }
                                    Err(err) => println!("Unimplemented AREQ: {:#X?}", err),
                                }
                            }
                            _ => panic!("incoming POLL or SREQ"),
                        },
                    }
                }
            },
        );
        Znp {
            tx: Mutex::new(Some(stx)),
            cbs: ctx,
        }
    }
    pub async fn sreq<S>(&mut self, req: S) -> Result<S::Srsp, SreqError>
    where
        S: Sreq + 'static,
    {
        // acquire writing rights
        let mut tx_lock = await!(self.tx.lock());
        // serial port can be gone if there has been an IO error before
        let sp_tx = tx_lock.take().ok_or_else(|| SreqError::SerialPortGone)?;
        let send = sp_tx.send(req.frame());
        let (cb_tx, cb_rx) = oneshot::channel();
        let register_callback = await!(self.cbs.clone().send(cb_tx));
        register_callback.map_err(|_| SreqError::SerialPortGone)?;
        let send_res = await!(send);
        let sp_tx = send_res.map_err(|err| SreqError::IO(err))?;
        let srsp = await!(cb_rx).map_err(|_| SreqError::SerialPortGone)?;
        let srsp = S::parse_res(srsp)
            .map_err(|err| SreqError::BadResponse(cmd::error::Error::from(err)))?;
        *tx_lock = Some(sp_tx);
        Ok(srsp)
    }
}
