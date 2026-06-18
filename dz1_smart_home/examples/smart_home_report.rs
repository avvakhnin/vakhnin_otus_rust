//! Пример исползования библиотеки
//! Создаёт экземпляр умного дома. Изменяет состояние одного из устройств
//! Выводит отчет до и после изменения
//! Для использования введите в корне проекта:
//! ```
//! cargo run --example smart_home_report
//! ```
use dz1_smart_home::{
    electro_socket::ElectroSocket, report::Report, smart_house::SmartHouse, smart_room,
    smart_tool::SmartTool, smart_tool_room::SmartToolRoom, term_detector::TermDetector,
};

fn main() {
    //Создаем экземпляр умного дома
    let mut smart_house = create_smart_house();
    //Выводим отчёт его состояния
    println!("{}", smart_house.get_report());
    //Тот же самый отчёт в форматированном виде для удобства
    //println!("{}", smart_house.get_report_pretty());

    //Выбираем первую комнату
    let smart_room = smart_house.get_mut("main");

    assert!(smart_room.is_some());

    //Выбираем второе устройство в комнате
    let smart_tool = smart_room.unwrap().get_mut("2");

    //Проверяем что устройство соответствует нашим ожиданиям - это должен быть выключатель
    assert!(
        matches!(smart_tool, Some(SmartTool::ElectroSocket { .. })),
        "Неожиданная конфигурация дома"
    );

    //Приводим устройство к нужному типу и переключаем состояние
    if let Some(SmartTool::ElectroSocket(e_socket)) = smart_tool {
        assert!(e_socket.is_switch_on());
        e_socket.switch_off();
        assert!(!e_socket.is_switch_on());
    }

    //Повторно выводим отчёт
    println!("{}", smart_house.get_report());
    //Тот же самый отчёт в форматированном виде для удобства
    //println!("{}", smart_house.get_report_pretty());
}

/// Декларативное создание умного дома с коллекцией комнат, в каждой из которых 0 и более умных устройств
fn create_smart_house() -> SmartHouse {
    let mut smart_house = SmartHouse::new();
    smart_house.insert(
        "main",
        smart_room!(
            "0" => ElectroSocket::new(false),
            "1" => TermDetector::default(),
            "2" => ElectroSocket::new(true),
            "3" => ElectroSocket::new(true),
        ),
    );
    smart_house.insert(
        "additional",
        smart_room!(
            "0" => TermDetector::default(),
            "1" => ElectroSocket::new(false),
        ),
    );
    smart_house.insert("empty", smart_room!());
    smart_house
}
