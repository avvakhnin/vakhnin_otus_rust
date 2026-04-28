//! Пример использщования библиотеки
//! Создаёт разные типы объектов, объединяет их, меняет состояние
//! Запуск cargo run

use dz1_smart_home::{
    electro_socket::ElectroSocket, smart_house::SmartHouse, smart_tool::SmartTool,
    smart_tool_room::SmartToolRoom, term_detector::TermDetector,
};

fn main() {
    let t = TermDetector::new("any");
    println!("Создали датчик {:?}", t);

    let e = ElectroSocket::new(true);
    println!("Создали умную розетку {:?}", e);

    let t_tool = SmartTool::TermDetector(t);
    println!("Обернули датчик в enum {:?}", t_tool);

    let e_tool = SmartTool::ElectroSocket(e);
    println!("Обернули розетку в enum{:?}", e_tool);

    let r = SmartToolRoom::new(vec![t_tool, e_tool]);
    println!("Создали комнату из двух умных приборо{:?}", r);

    let mut h = SmartHouse::new(vec![r]);
    println!("Создали умный дом из одной комнаты{:?}", h);

    h.report();

    //Получаем сслыку на комнату
    let r = h.get_mut(0);
    //Получаем ссылку на прибор
    let tool = r.get_mut(1);
    //Измлекаем тип из enum
    if let SmartTool::ElectroSocket(e) = tool {
        //Выключаем розетку
        e.switch_off();
    }
    h.report();
}
