use threaded_iterators::{NetworkData, consume_data_in_thread, process_iterator_in_thread, udp_listener};

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
    let handler = consume_data_in_thread(nondeterminstic_handler);
    for data in process_iterator_in_thread(udp_listener(12345)) {
        handler(data);
    }
}
