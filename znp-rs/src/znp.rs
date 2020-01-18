use super::areq::AreqOut;
use super::sreq::Sreq;
use super::znp_codec;
use crate::cmd;
use futures_util::{future, stream, SinkExt, StreamExt};
use std::path::Path;
use std::time::Duration;
use tokio::sync::{mpsc, oneshot};
use tokio::time::timeout;
use tokio_serial::{Serial, SerialPortSettings};
use znp_codec::{Subsys, ZnpCmd, ZnpCodec};

#[derive(Debug)]
pub enum SreqError {
    BadResponse(cmd::error::Error),
    SerialPortGone,
    TimedOut,
    IO(std::io::Error),
}
#[derive(Debug)]
pub enum AreqError {
    IO(std::io::Error),
}
#[derive(Debug)]
struct Callback {
    cb: oneshot::Sender<ZnpCmd>,
    subsys: Subsys,
    cmd_id: u8,
}
enum SendJob {
    Sreq(ZnpCmd, Callback),
    Areq(ZnpCmd),
}
async fn receiver(
    cbs_rx: mpsc::Receiver<Callback>,
    mut sp_rx: futures_util::stream::SplitStream<tokio_util::codec::Framed<Serial, ZnpCodec>>,
    mut areq_tx: mpsc::Sender<crate::cmd::Areq>,
) {
    let mut cbs_rx = cbs_rx.filter(|cb| future::ready(!cb.cb.is_closed()));
    while let Some(frame) = sp_rx.next().await {
        use znp_codec::Type::{AREQ, SRSP};
        match frame {
            Err(err) => {
                eprintln!("{}", err);
                break;
            }
            Ok(frame) => match frame.typ() {
                SRSP => {
                    let cb = loop {
                        match timeout(Duration::from_millis(100), cbs_rx.next()).await {
                            Err(err) => {
                                panic!(err);
                            }
                            // Ok(Async::NotReady) => {
                            //     eprintln!("Unexpected SRSP: {:?}", frame);
                            //     panic!("SRSP no one was waiting for");
                            // }
                            Ok(None) => {
                                panic!("callback sender is finished and closed");
                            }
                            Ok(Some(Callback { cb, subsys, cmd_id })) => {
                                if subsys != frame.subsys() || cmd_id != frame.cmd_id() {
                                    eprintln!("Mismatched SRSP, probably old: {:?}", frame);
                                    continue;
                                } else {
                                    break cb;
                                }
                            }
                        }
                    };
                    let cb_res = cb.send(frame);
                    if let Err(frame) = cb_res {
                        eprintln!("Late SRSP, dropping: {:?}", frame);
                        // TODO: Determine if this is the next SRSP, or a late one.
                    }
                }
                AREQ => {
                    use crate::cmd::Areq;
                    match Areq::from_subsys(frame) {
                        Ok(areq) => {
                            (&mut areq_tx).send(areq).await.expect("Couldn't send AREQ");
                        }
                        Err(cmd::error::Error::Unimplemented { subsys, cmd_id }) => {
                            println!("Unimplemented AREQ: {:?} Cmd1 = {:#X?}", subsys, cmd_id)
                        }
                        Err(err) => println!("Unimplemented AREQ: {:#X?}", err),
                    }
                }
                _ => panic!("incoming POLL or SREQ"),
            },
        }
    }
}
pub struct Sender {
    sp_tx: stream::SplitSink<tokio_util::codec::Framed<Serial, ZnpCodec>, ZnpCmd>,
    cbs_tx: mpsc::Sender<Callback>,
}
impl Sender {
    pub fn from_path<P>(path: P) -> (Self, mpsc::Receiver<crate::cmd::Areq>)
    where
        P: AsRef<Path>,
    {
        let sp_settings = SerialPortSettings {
            baud_rate: 115_000,
            ..Default::default() // 8-N-1 is default
        };
        let sp = Serial::from_path(path, &sp_settings).unwrap();
        let sp = tokio_util::codec::Framed::new(sp, ZnpCodec);
        let (cbs_tx, cbs_rx) = mpsc::channel::<Callback>(2);
        let (areq_tx, areq_rx) = mpsc::channel::<crate::cmd::Areq>(1);
        let (sp_tx, sp_rx) = sp.split();
        tokio::spawn(receiver(cbs_rx, sp_rx, areq_tx));
        (Sender { sp_tx, cbs_tx }, areq_rx)
    }
    pub async fn sreq<S>(&mut self, req: S) -> Result<S::Srsp, SreqError>
    where
        S: Sreq + 'static,
    {
        let (cb_tx, cb_rx) = oneshot::channel();
        let frame = req.frame();
        let cb = Callback {
            cb: cb_tx,
            subsys: frame.subsys(),
            cmd_id: frame.cmd_id(),
        };
        self.cbs_tx.send(cb).await.expect("receiver gone");
        self.sp_tx.send(frame).await.expect("SREQ send IO error");
        let cb_rx = timeout(Duration::from_millis(1000), cb_rx);
        let srsp = cb_rx
            .await
            .map_err(|err| SreqError::TimedOut)?
            .map_err(|err| SreqError::SerialPortGone)?;
        let srsp = S::parse_res(srsp).map_err(SreqError::BadResponse)?;
        Ok(srsp)
    }
    pub async fn areq<A>(&mut self, req: A)
    where
        A: AreqOut + 'static,
    {
        self.sp_tx
            .send(req.frame())
            .await
            .expect("AREQ send IO error");
    }
}
