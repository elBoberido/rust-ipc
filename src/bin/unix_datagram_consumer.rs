use std::str::FromStr;

use ipc::get_payload;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let data_size = usize::from_str(&args[1]).unwrap();

    let is_child = true;
    let socket_wrapper = ipc::unix_datagram::UnixDatagramWrapper::new(is_child, data_size);
    socket_wrapper.connect_to_peer();

    let (_request_data, response_data) = get_payload(data_size);

    loop {
        let _request = socket_wrapper.recv();
        socket_wrapper.send(&response_data);
    }
}
