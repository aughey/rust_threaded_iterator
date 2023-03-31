use std::{net::{SocketAddr, UdpSocket}, sync::mpsc::Receiver};

struct NetworkData {
    data: Vec<u8>,
    src: SocketAddr,
}

fn udp_listner(port: u16) -> impl Iterator<Item = NetworkData> {
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

fn process_iterator_in_thread<T: Send + 'static, It: Iterator<Item=T> + Send + 'static>(iterator: It) -> Receiver<T> {
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

fn consume_data_in_thread<T : Send + 'static>(handler: impl Fn(T) + Send + 'static) -> impl Fn(T) {
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

fn nondeterminstic_handler(data: NetworkData) {
    println!("Received {} bytes from {}", data.data.len(), data.src);
    let data = data.data;
    // convert data to a string
    let data = String::from_utf8(data).unwrap();
    println!("Data: {:?}", data);
    // sleep for a random time
    let random_time = rand::random::<u64>() % 500000;
    std::thread::sleep(std::time::Duration::from_micros(random_time));
}

fn main() {
    println!("Starting UDP listener on port 12345");
    let handler = consume_data_in_thread(nondeterminstic_handler);
    for data in process_iterator_in_thread(udp_listner(12345)) {
        handler(data);
    }
}
