use std::assert_matches;

use dz1_smart_home::{
    electro_socket::ElectroSocket, report::Report, smart_house::SmartHouse, smart_room,
    smart_tool::SmartTool, smart_tool_room::SmartToolRoom, term_detector::TermDetector,
};

fn main() {
    //Предсоздаём набор приборов
    let td = TermDetector::default();
    let s1 = ElectroSocket::new(true);
    let s2 = ElectroSocket::new(false);

    //Создаём пустой дом
    let mut home = SmartHouse::new();
    //Добавляем комнаты созданные разными способами
    home.insert(
        "room 1",
        smart_room!("term" => td, "socket 1" => s1, "socket 2" => s2),
    );
    home.insert("room 2", SmartToolRoom::default());
    assert_eq!(2, home.size());

    //Печать отчёта до изменений
    print_report(&home);

    //Удаляем комнату и проверяем что осталось
    let room = home.remove("room 1");
    assert_eq!(1, home.size());

    //проверяем что вернулась правильная комната
    assert!(room.is_some());
    let mut room = room.expect("");
    assert_eq!(3, room.size());

    //Добавляем устройство в комнату
    room.insert("socket 3", ElectroSocket::new(false));
    assert_eq!(4, room.size());

    //Удаляем устройство из комнаты
    let tool = room.remove("socket 1");
    assert_eq!(3, room.size());

    //Проверяем что вернулось правильное устройство
    assert!(tool.is_some());
    let tool = tool.expect("");
    assert_matches!(tool, SmartTool::ElectroSocket(_));

    //Печатаем отчёты о полученных объектах
    print_report(&home);
    print_report(&room);
    print_report(&tool);
}

fn print_report<R: Report>(target: &R) {
    println!("{}", target.get_report());
}
