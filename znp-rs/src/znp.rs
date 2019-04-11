use super::areq::AreqOut;
use super::sreq::Sreq;
use super::znp_codec;
use crate::cmd;
use std::path::Path;
use std::time::Duration;
use tokio::prelude::*;
use tokio::sync::{mpsc, oneshot};
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
    mut sp_rx: stream::SplitStream<tokio::codec::Framed<Serial, ZnpCodec>>,
) {
    let mut cbs_rx = cbs_rx.filter(|cb| !cb.cb.is_closed());
    while let Some(frame) = await!(sp_rx.next()) {
        use znp_codec::Type::{AREQ, SRSP};
        match frame {
            Err(err) => {
                eprintln!("{}", err);
                break;
            }
            Ok(frame) => match frame.typ() {
                SRSP => {
                    let cb = loop {
                        match cbs_rx.poll() {
                            Err(err) => {
                                panic!(err);
                            }
                            Ok(Async::Ready(None)) => {
                                panic!("callback sender is finished and closed");
                            }
                            Ok(Async::NotReady) => {
                                eprintln!("Unexpected SRSP: {:?}", frame);
                                panic!("SRSP no one was waiting for");
                            }
                            Ok(Async::Ready(Some(Callback { cb, subsys, cmd_id }))) => {
                                if subsys != frame.subsys() || cmd_id != frame.cmd_id() {
                                    println!("Mismatched SRSP, probably old");
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
                        Ok(areq) => println!("Known AREQ: {:?}", areq),
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
async fn sender(
    mut sendjob_rx: mpsc::Receiver<SendJob>,
    mut sp_tx: stream::SplitSink<tokio::codec::Framed<Serial, ZnpCodec>>,
    mut cbs_tx: mpsc::Sender<Callback>,
) {
    // None is sender closed, RecvError is sender closed.
    while let Some(Ok(sendjob)) = await!(sendjob_rx.next()) {
        match sendjob {
            SendJob::Sreq(frame, cb) => {
                await!((&mut cbs_tx).send(cb)).expect("receiver gone");
                let send = Sink::send(&mut sp_tx, frame);
                await!(send).expect("SREQ send IO error");
            }
            SendJob::Areq(frame) => {
                let send = Sink::send(&mut sp_tx, frame);
                await!(send).expect("AREQ send IO error");
            }
        }
    }
}
pub struct Znp {
    tx: mpsc::Sender<SendJob>,
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
        let (cbs_tx, cbs_rx) = mpsc::channel::<Callback>(2);
        let (sendjob_tx, sendjob_rx) = mpsc::channel::<SendJob>(1);
        let (sp_tx, sp_rx) = sp.split();
        tokio::spawn_async(receiver(cbs_rx, sp_rx));
        tokio::spawn_async(sender(sendjob_rx, sp_tx, cbs_tx));
        Znp { tx: sendjob_tx }
    }
    pub async fn sreq<S>(&mut self, req: S) -> Result<S::Srsp, SreqError>
    where
        S: Sreq + 'static,
    {
        let (cb_tx, cb_rx) = oneshot::channel();
        let frame = req.frame();
        let cb_tx = Callback {
            cb: cb_tx,
            subsys: frame.subsys(),
            cmd_id: frame.cmd_id(),
        };
        let send = Sink::send(&mut self.tx, SendJob::Sreq(frame, cb_tx));
        await!(send).expect("Error placing SREQ job");
        let cb_rx = cb_rx.timeout(Duration::from_millis(1000));
        let srsp = await!(cb_rx).map_err(|err| {
            if err.is_elapsed() {
                SreqError::TimedOut
            } else if err.is_inner() {
                SreqError::SerialPortGone
            } else {
                panic!(err);
            }
        })?;
        let srsp = S::parse_res(srsp).map_err(|err| SreqError::BadResponse(From::from(err)))?;
        Ok(srsp)
    }
    pub async fn areq<A>(&mut self, req: A)
    where
        A: AreqOut + 'static,
    {
        let send = Sink::send(&mut self.tx, SendJob::Areq(req.frame()));
        await!(send).expect("Error placing AREQ job");
    }
}
