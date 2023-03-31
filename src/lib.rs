use std::{net::{SocketAddr, UdpSocket}, sync::mpsc::Receiver};

pub struct NetworkData {
    pub data: Vec<u8>,
    pub src: SocketAddr,
}

pub fn udp_listener(port: u16) -> impl Iterator<Item = NetworkData> {
    let socket = UdpSocket::bind(("0.0.0.0", port)).unwrap();

    std::iter::from_fn(move || {
        let mut buf = [0; 65536];
        println!("Waiting for data");
        let (size, src) = socket.recv_from(&mut buf).unwrap();
        Some(NetworkData {
            data: buf[..size].to_vec(),
            src,
        })
    })
}

pub fn process_iterator_in_thread<T: Send + 'static, It: Iterator<Item=T> + Send + 'static>(iterator: It) -> Receiver<T> {
    // create a spsc channel
    let (tx, rx) = std::sync::mpsc::channel();
    // spawn a thread
    std::thread::spawn(move || {
        // iterate over the iterator
        for item in iterator {
            // send the item to the channel
            tx.send(item).unwrap();
        }
    });
    // return the receiver
    rx
}

pub fn consume_data_in_thread<T : Send + 'static>(handler: impl Fn(T) + Send + 'static) -> impl Fn(T) {
    let (tx, rx) = std::sync::mpsc::channel();

    std::thread::spawn(move || {
        for data in rx {
            handler(data);
        }
    });

    move |data| {
        tx.send(data).unwrap();
    }
}