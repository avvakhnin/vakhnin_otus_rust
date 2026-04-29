//! Пример исползования библиотеки
//! Создаёт экземпляр умного дома. Изменяет состояние одного из устройств
//! Выводит отчет до и после изменения
//! Для использования введите в корне проекта:
//! ```
//! cargo run --example smart_home_report
//! ```
use dz1_smart_home::{
    electro_socket::ElectroSocket, smart_house::SmartHouse, smart_tool::SmartTool,
    smart_tool_room::SmartToolRoom, term_detector::TermDetector,
};

fn main() {
    //Создаем экземпляр умного дома
    let mut smart_house = create_smart_house();
    //Выводим отчёт его состояния
    smart_house.report();
    //Тот же самый отчёт в форматированном виде для удобства
    //println!("{:#?}", smart_house);

    //Выбираем первую комнату
    let smart_room = smart_house.get_mut(0);
    //Выбираем второе устройство в комнате
    let smart_tool = smart_room.get_mut(2);

    //Проверяем что устройство соответствует нашим рожиданиям - это должен быть выключатель
    assert!(
        matches!(smart_tool, SmartTool::ElectroSocket { .. }),
        "Неожиданная конфигурация дома"
    );

    //Приводим устройство к нужному типу и переключаем состояние
    if let SmartTool::ElectroSocket(e_socket) = smart_tool {
        assert!(e_socket.is_switch_on());
        e_socket.switch_off();
        assert!(!e_socket.is_switch_on());
    }

    //Повторно выводим отчёт
    smart_house.report();
    //Тот же самый отчёт в форматированном виде для удобства
    //println!("{:#?}", smart_house);
}

/// Декларативное создание умного дома с коллекцие комнат, в каждой из которых 0 и более умных устройств
fn create_smart_house() -> SmartHouse {
    SmartHouse::new(vec![
        SmartToolRoom::new(vec![
            SmartTool::ElectroSocket(ElectroSocket::new(false)),
            SmartTool::TermDetector(TermDetector::new("detector 1")),
            SmartTool::ElectroSocket(ElectroSocket::new(true)),
            SmartTool::ElectroSocket(ElectroSocket::new(true)),
        ]),
        SmartToolRoom::new(vec![
            SmartTool::TermDetector(TermDetector::new("detector 2")),
            SmartTool::ElectroSocket(ElectroSocket::new(false)),
        ]),
        SmartToolRoom::new(vec![]),
    ])
}
