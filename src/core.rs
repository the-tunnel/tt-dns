extern crate rand;
use std::thread;
use std::process;
use std::sync::{Arc, Mutex};
use std::error::Error;
use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};

pub fn run(LISTEN: &str, UPSTREAM: &str) -> std::io::Result<()> {
//        let LISTEN = LISTEN.split("//").collect::<Vec<&str>>()[1];
//        let UPSTREAM = UPSTREAM.split("//").collect::<Vec<&str>>()[1];

        let socket_listen = UdpSocket::bind(LISTEN).unwrap_or_else(|err|{
            eprintln!("Error binding: [{}], {}", LISTEN, err);
            process::exit(-1);
        });
        let socket_upstream = UdpSocket::bind("0.0.0.0:0")?;
        //socket_upstream.connect(UPSTREAM)?;

        let map: HashMap<u16, SocketAddr> = HashMap::with_capacity(65536);
        let map = Arc::new(Mutex::new(map));

        let _map = map.clone();
        let _socket_listen = socket_listen.try_clone().unwrap();
        let _socket_upstream = socket_upstream.try_clone().unwrap();
        let _UPSTREAM = UPSTREAM.to_owned();
        let upstream = thread::spawn(move || {
            let mut buf = [0u8; 2048];
            loop {
                let (len, addr) = _socket_listen.recv_from(&mut buf).unwrap();
                let trans_id = ((buf[0] as u16) << 8) + buf[1] as u16;
                //if map.contains_key(&trans_id) {
                    // do something
                //}
                //else {
                    _map.lock().unwrap().insert(trans_id, addr);
                //}
                let new_len = append_OPT_record(&mut buf, len).unwrap();
                _socket_upstream.send_to(&buf[..new_len], &_UPSTREAM).unwrap_or_else(|err|{
                    eprintln!("Error sending to upstream: {}", err);
                    0
                });
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
                    if let Some(addr) = _map.lock().unwrap().get(&trans_id) {
                        //let new_len = strip_OPT_record(&mut buf, len).unwrap();
                        _socket_listen.send_to(&buf[..len], addr).unwrap_or_else(|err|{
                            eprintln!("Error sending to client: {}", err);
                            0
                        });
                    }
                }
            }
        });

        upstream.join().unwrap();
        downstream.join().unwrap();
        Ok(())
}

pub fn check_OPT_record(packet: &[u8], _len: usize) -> bool {
    let additional_RRs = ((packet[10] as u16) << 8) + packet[11] as u16;
    if additional_RRs == 0 {
        return false
    }
    // we shall strictly check the OPT record, but it works for now.
    true
}

pub fn append_OPT_record(packet: &mut [u8], len: usize) -> Result<usize, Box<dyn Error>> {
    if check_OPT_record(&packet, len) {     // skip if there is an OPT record already
        return Ok(len)
    }

    packet[11] += 1;            // additional_RRs ++
    // 12 (0x0c) bytes of record = OPTION Code: 0x000a (cookie) + OPTION len: 0x0008
    // 8 bytes of COOKIE should be random
    packet[len..len+15].copy_from_slice(&[0x00,0x00,0x29,0x10,0x00,0x00,0x00,0x00,0x00,0x00,0x0c, 0x00,0x0a,0x00,0x08]);
    let mut count = 0;
    while count < 8 {
        packet[len+11+4+count] = rand::random::<u8>();
        count += 1;
    }
    Ok(len+11+4+8)
}
