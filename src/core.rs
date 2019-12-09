#![allow(dead_code)]
#![allow(non_camel_case_types)]

use std::thread;
use std::sync::{Arc, Mutex};
use std::error::Error;
use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};

struct TT_DNS_STATE {
    client_addr : SocketAddr,
//    forwarded   : bool,
//    finished    : bool,
//    request_buf : Vec<u8>,
//    request_len : u16,
//    response_buf: Vec<u8>,
//    response_len: u16,
}

pub fn run(LISTEN: &str, UPSTREAM: &str) -> std::io::Result<()> {
//        let LISTEN = LISTEN.split("//").collect::<Vec<&str>>()[1];
//        let UPSTREAM = UPSTREAM.split("//").collect::<Vec<&str>>()[1];

        let socket_listen = UdpSocket::bind(LISTEN)?;
        let socket_upstream = UdpSocket::bind("0.0.0.0:0")?;
        socket_upstream.connect(UPSTREAM)?;

        let map: HashMap<u16, TT_DNS_STATE> = HashMap::with_capacity(65536);
        let map = Arc::new(Mutex::new(map));


        let _map = map.clone();
        let _socket_listen = socket_listen.try_clone().unwrap();
        let _socket_upstream = socket_upstream.try_clone().unwrap();
        let upstream = thread::spawn(move || {
            let mut buf = [0u8; 2048];

            loop {
                let (len, addr) = _socket_listen.recv_from(&mut buf).unwrap();
                let trans_id = ((buf[0] as u16) << 8) + buf[1] as u16;
                let tt_dns = TT_DNS_STATE { client_addr: addr};
                //if map.contains_key(&trans_id) {
                    // do something
                //}
                //else {
                    _map.lock().unwrap().insert(trans_id, tt_dns);
                //}

                let new_len = append_OPT_record(&mut buf, len).unwrap();
                _socket_upstream.send(&buf[..new_len]).unwrap();
            }
        });

        let _map = map.clone();
        let _socket_listen = socket_listen.try_clone().unwrap();
        let _socket_upstream = socket_upstream.try_clone().unwrap();
        let downstream = thread::spawn(move || {
            let mut buf = [0u8; 2048];
            loop {
                let len = _socket_upstream.recv(&mut buf).unwrap();
                let trans_id = ((buf[0] as u16) << 8) + buf[1] as u16;
                if check_OPT_record(&buf, len) {
                    if let Some(tt_dns) = _map.lock().unwrap().get(&trans_id) {
                        //let new_len = strip_OPT_record(&mut buf, len).unwrap();
                        _socket_listen.send_to(&buf[..len], tt_dns.client_addr).unwrap();
                    }
                }
            }
        });

        upstream.join().unwrap();
        downstream.join().unwrap();
        Ok(())
}

pub fn check_OPT_record(packet: &[u8], len: usize) -> bool {
    true
}

pub fn append_OPT_record(packet: &mut [u8], len: usize) -> Result<usize, Box<dyn Error>> {
    Ok(0)
}

pub fn strip_OPT_record(packet: &mut [u8], len: usize) -> Result<usize, Box<dyn Error>> {
    Ok(0)
}
