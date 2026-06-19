use std::assert_matches;

use dz1_smart_home::core::{
    electro_socket::ElectroSocket, smart_house::SmartHouse, smart_house_error::SmartHouseError,
    smart_tool_room::SmartToolRoom, term_detector::TermDetector,
};
use dz1_smart_home::smart_room;

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

    //Пытаемся получить существующее устройство в доме
    let get_result = home.get_smart_tool("room 1", "socket 1");
    assert!(get_result.is_ok());
    if let Ok(res) = get_result {
        println!("Получили результат {:?}", res);
    }

    //Пытаемся получить несуществующее устройство в доме
    let get_result = home.get_smart_tool("room 1", "socket 5");
    assert!(get_result.is_err());
    if let Err(error) = get_result {
        assert_matches!(error, SmartHouseError::ToolNotFound(_));
        println!("Получили ошибку {}", error);
    }

    //Пытаемся получить несуществующее устройство в доме
    let get_result = home.get_smart_tool("room 3", "socket 1");
    assert!(get_result.is_err());
    if let Err(error) = get_result {
        assert_matches!(error, SmartHouseError::RoomNotFound(_));
        println!("Получили ошибку {}", error);
    }
}
