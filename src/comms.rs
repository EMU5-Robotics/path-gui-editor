use communication::{client::Client, packet::ToRobot, ToClient};
use eframe::egui::Context;
use std::sync::mpsc::{self, Receiver, Sender};

// currently only supports receiving data
pub struct Comms {
    recv: Receiver<ToClient>,
    send: Sender<ToRobot>,
}

impl Comms {
    pub fn new(addr: &str, ctx: Context) -> Self {
        let (send_main, recv_from_thread) = mpsc::channel();
        let (send_thread, recv_from_main) = mpsc::channel();

        let addr = addr.to_owned();
        std::thread::spawn(move || {
            let mut was_connected = false;
            let mut op_client = Client::new(&addr).ok();
            loop {
                let Some(ref mut client) = op_client else {
                    op_client = Client::new(&addr).ok();
                    std::thread::yield_now();
                    continue;
                };
                let data = match client.receive_data() {
                    Ok(d) => {
                        if !was_connected {
                            log::info!("Connection with robot established.");
                            was_connected = true;
                        }
                        d
                    }
                    Err(communication::packet::Error::Io(_)) => {
                        if was_connected {
                            log::warn!("Connection with robot disconnected.");
                            op_client = None;
                            was_connected = false;
                        }
                        continue;
                    }
                    Err(e) => {
                        log::error!("got {e} when trying to receive data from robot.");
                        continue;
                    }
                };
                let l = data.len();
                for d in data {
                    send_main.send(d).unwrap();
                }
                if let Ok(pkt) = recv_from_main.try_recv() {
                    if let Err(e) = client.send_request(&pkt) {
                        log::warn!("Failed to send packet {pkt:?} to robot with {e}");
                    }
                }

                // until we get a smarter way to do this
                if l != 0 {
                    ctx.request_repaint();
                }
                std::thread::yield_now();
            }
        });
        Self {
            recv: recv_from_thread,
            send: send_thread,
        }
    }
    pub fn get_packets(&self) -> Vec<ToClient> {
        let mut pkts = Vec::new();
        while let Ok(v) = self.recv.try_recv() {
            pkts.push(v);
        }
        pkts
    }
    pub fn send_packet(&self, pkt: ToRobot) {
        if let Err(e) = self.send.send(pkt) {
            log::error!("Failed to send pkt to client thread with {e}");
        }
    }
}
