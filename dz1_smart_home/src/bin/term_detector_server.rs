use std::{net::UdpSocket, thread, time::Duration};

use log::{debug, info};

fn main() {
    env_logger::init();
    let args: Vec<String> = std::env::args().collect();
    let binding = "127.0.0.1:4334".to_string();
    let addr = args.get(1).unwrap_or(&binding);
    info!("Запускаем UDP сервер по адресу {}", addr);
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Не удалось запустить UDP сервер");
    debug!("Сервер успешно запущен");
    socket.send_to("HALLO\r\n".as_bytes(), addr);
    thread::sleep(Duration::from_secs(5));
    let mut response = [0u8; 6];
    socket.recv(&mut response);
}
