use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
};

use log::{debug, error, info};

/// Запускает TCP сервер по адресу переданному в командной строке
/// Пример запуска:
/// cargo run smart_home_confirmation 127.0.0.1:9876
/// Если не передать адресс, то сервер поднимется по адресу 127.0.0.1:4321
/// Сервер принимает следующие запросы в виде набора байтов:
/// "switch_on_" - команда на включение розетки, в ответ ничего не вернётся
/// "switch_off" - команда на выключение розетки, в ответ ничего не вернётся
/// "get_status" - клиенту вернётся значение 0000 если розетка выключена, 0001 если розетка включена
/// "get_power_" - клиенту вернётся значение мощности розетки в Вт в виде четырёхзначного числа
fn main() {
    env_logger::init();

    let args: Vec<String> = std::env::args().collect();
    let binding = "127.0.0.1:4321".to_string();
    let addr = args.get(2).unwrap_or(&binding);
    info!("Запускаем TCP сервер по адресу {}", addr);
    let listener = TcpListener::bind(addr).expect("Не удалось запустить TCP сервер");
    debug!("Сервер успешно запущен");

    let switch_state = Arc::new(AtomicBool::new(false));

    for stream in listener.incoming() {
        debug!("Приняли соединение");
        let stream = match stream {
            Ok(stream) => {
                if let Ok(peer_addr) = stream.peer_addr() {
                    info!("Соединение от клиента {}", peer_addr);
                } else {
                    info!("Успешное соединение")
                }
                stream
            }
            Err(err) => {
                error!("Не удалось обработать соединение {}", err);
                continue;
            }
        };

        let switch_state = switch_state.clone();

        thread::spawn(|| handle_connection(switch_state, stream));
    }
}

fn handle_connection(switch_state: Arc<AtomicBool>, mut stream: TcpStream) {
    let mut buf = [0; 10];
    stream.read_exact(&mut buf).unwrap();

    let request = String::from_utf8(buf.to_vec()).expect("Некорректный запрос");
    debug!("Получили запрос \"{}\"", request);

    match request.as_str() {
        "switch_on_" => {
            switch_state.store(true, Ordering::SeqCst);
            info!("Включили розетку")
        }
        "switch_off" => {
            switch_state.store(false, Ordering::SeqCst);
            info!("Выключили розетку")
        }
        "get_status" => {
            debug!("Отправляем клиенту состояние розетки");
            if !switch_state.load(Ordering::SeqCst) {
                stream
                    .write_all("0000".as_bytes())
                    .expect("Ошибка отправки");
            } else {
                stream
                    .write_all("0001".as_bytes())
                    .expect("Ошибка отправки");
            }
        }
        "get_power_" => {
            let power = if switch_state.load(Ordering::SeqCst) {
                rand::random_range(1..4000)
            } else {
                0
            };
            debug!(
                "Отправляем клиенту значение мощности розетки - {:04}",
                power
            );
            stream
                .write_all(format!("{:04}", power).as_bytes())
                .expect("Ошибка отправки")
        }
        _ => panic!("Некорректный запрос"),
    };
    if switch_state.load(Ordering::SeqCst) {
        debug!("Статус: розетка включена");
    } else {
        debug!("Статус: розетка выключена");
    }
}
