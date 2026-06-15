//! Комната, содержащая массив умных устройств
use std::collections::HashMap;

use crate::smart_tool::SmartTool;
#[derive(Debug)]
pub struct SmartToolRoom {
    smart_tools: HashMap<&'static str, SmartTool>,
}

impl SmartToolRoom {
    ///Создает комнату со списком умных устройств
    pub fn new() -> SmartToolRoom {
        SmartToolRoom {
            smart_tools: HashMap::new(),
        }
    }

    ///Возвращает количество устройств в комнате
    pub fn size(&self) -> usize {
        self.smart_tools.len()
    }

    ///Добавляет устройство в дом под уникальным именемш
    pub fn insert(&mut self, name: &'static str, tool: SmartTool) -> Option<SmartTool> {
        self.smart_tools.insert(name, tool)
    }

    ///Возвращает ссылку на устройство по указанному индексу
    pub fn get(&self, name: &'static str) -> Option<&SmartTool> {
        self.smart_tools.get(name)
    }

    ///Возвращает мутабельную ссылку на устройство по указанному индексу.
    pub fn get_mut(&mut self, name: &'static str) -> Option<&mut SmartTool> {
        self.smart_tools.get_mut(name)
    }
    /// Выводит в стандартный вывод отчёт о всех устройствах в комнате.
    pub fn report(&self) {
        println!("{:?}", self);
    }
}

///Макрос для создания SmartRoom с парами ключ => значение
macro_rules! smart_room {
    () => {
        SmartToolRoom::new()
    };

    ($($key:expr => $value:expr),* $(,)?) => {{
        let mut room = SmartToolRoom::new();
        $(room.insert($key, $value);)*
        room}
    };
}

pub(crate) use smart_room;

#[cfg(test)]
mod tests {
    use crate::{
        electro_socket::ElectroSocket, smart_tool::SmartTool, smart_tool_room::SmartToolRoom,
        term_detector::TermDetector,
    };
    use std::{assert_matches, panic};

    fn setup() -> SmartToolRoom {
        smart_room!("detector" => SmartTool::TermDetector(TermDetector::new("detector")),
            "socket1" => SmartTool::ElectroSocket(ElectroSocket::new(false)),
            "socket2" => SmartTool::ElectroSocket(ElectroSocket::new(true)))
    }

    #[test]
    fn test_new() {
        let result = panic::catch_unwind(SmartToolRoom::new);
        assert!(result.is_ok(), "Код не должен паниковать");
        assert_eq!(0, result.unwrap().size(), "Некорректно создан объект");
    }

    #[test]
    fn test_size() {
        let r = setup();
        assert_eq!(
            3,
            r.size(),
            "Некорректно определёно количеставо датчиков в комнате"
        );
    }

    #[test]
    fn test_insert() {
        let mut room = SmartToolRoom::new();
        room.insert("new", SmartTool::ElectroSocket(ElectroSocket::new(true)));
        assert_eq!(1, room.size(), "Неверно отработала вставка");
    }

    #[test]
    fn test_get() {
        let r = setup();
        let t = r.get("socket1");
        assert_matches!(
            t,
            Some(SmartTool::ElectroSocket(e)) if !e.is_switch_on(),
            "Возвращен неверный элемент"
        );
    }

    #[test]
    fn test_get_none() {
        let r = setup();
        let t = r.get("camera");
        assert!(t.is_none(), "Возвращен неверный элемент");
    }

    #[test]
    fn test_get_mut() {
        let mut r = setup();
        let t = r.get_mut("socket2");

        assert_matches!(
            t,
            Some(SmartTool::ElectroSocket(e)) if e.is_switch_on(),
            "Возвращен неверный элемент"
        );
    }

    #[test]
    fn test_get_mut_none() {
        let mut r = setup();
        let t = r.get_mut("micro");
        assert!(t.is_none(), "Возвращен неверный элемент");
    }

    #[test]
    fn test_report() {
        let r = setup();
        let result = panic::catch_unwind(|| r.report());

        assert!(result.is_ok(), "Код не должен паниковать");
    }
}
