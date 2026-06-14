//! Умный дом, содержащий массив комнат.
use std::collections::HashMap;

use crate::smart_tool_room::SmartToolRoom;
#[derive(Debug, Default)]
pub struct SmartHouse {
    rooms: HashMap<&'static str, SmartToolRoom>,
}

impl SmartHouse {
    ///Создаёт умный дом со списком комнгат с умными устройствами
    pub fn new() -> Self {
        SmartHouse {
            rooms: HashMap::new(),
        }
    }

    ///Возвращает количество комнат в доме
    pub fn size(&self) -> usize {
        self.rooms.len()
    }

    pub fn insert(&mut self, name: &'static str, room: SmartToolRoom) -> Option<SmartToolRoom> {
        self.rooms.insert(name, room)
    }

    ///Возвращает ссылку на комнату по указанному индексу.
    pub fn get(&self, name: &'static str) -> Option<&SmartToolRoom> {
        self.rooms.get(name)
    }

    ///Возвращает мутабельную ссылку на комнату по указанному индексу.
    pub fn get_mut(&mut self, name: &'static str) -> Option<&mut SmartToolRoom> {
        self.rooms.get_mut(name)
    }

    ///Выводит в стандартный вывод отчёт о всех комнатах.
    pub fn report(&self) {
        println!("{:?}", self);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        electro_socket::ElectroSocket, smart_house::SmartHouse, smart_tool::SmartTool,
        smart_tool_room::SmartToolRoom, term_detector::TermDetector,
    };
    use std::{assert_matches, panic};

    fn setup() -> SmartHouse {
        let mut house = SmartHouse::new();
        let st1 = SmartTool::TermDetector(TermDetector::new("detector 1"));
        let st2 = SmartTool::ElectroSocket(ElectroSocket::new(false));
        let st3 = SmartTool::ElectroSocket(ElectroSocket::new(true));
        house.insert("first", SmartToolRoom::new(vec![st1, st2, st3]));

        let st1 = SmartTool::ElectroSocket(ElectroSocket::new(true));
        let st2 = SmartTool::TermDetector(TermDetector::new("detector 2"));
        house.insert("additional", SmartToolRoom::new(vec![st1, st2]));
        house
    }

    #[test]
    fn test_new() {
        let result = panic::catch_unwind(|| SmartHouse::new());

        assert!(result.is_ok(), "Код не должен паниковать");
        assert_eq!(0, result.unwrap().size(), "Не корректно создан объект");
    }

    #[test]
    fn test_size() {
        let h = setup();
        assert_eq!(
            2,
            h.size(),
            "Некорректно определёно количеставо комнат в доме"
        );
    }

    #[test]
    fn test_insert() {
        let mut house = SmartHouse::new();
        house.insert("new", SmartToolRoom::new(vec![]));
        assert_eq!(1, house.size(), "Не верно отработала вставка");
    }

    #[test]
    fn test_get() {
        let h = setup();
        let r = h.get("additional");
        assert_matches!(r, Some(room) if room.size() == 2, "Возвращен неверный элемент");
    }

    #[test]
    fn test_get_none() {
        let h = setup();
        let r = h.get("bathroom");
        assert!(r.is_none());
    }

    #[test]
    fn test_get_mut() {
        let mut h = setup();
        let r = h.get_mut("first");
        assert_matches!(r, Some(room) if room.size() == 3, "Возвращен неверный элемент");
    }

    #[test]
    fn test_get_mut_none() {
        let mut h = setup();
        let r = h.get_mut("hall");
        assert!(r.is_none());
    }

    #[test]
    fn test_report() {
        let r = setup();
        let result = panic::catch_unwind(|| {
            r.report();
        });

        assert!(result.is_ok(), "Код не должен паниковать");
    }
}
