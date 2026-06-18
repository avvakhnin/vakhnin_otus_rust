//! Умный дом, содержащий массив комнат.
use std::collections::HashMap;

use crate::{
    report::Report, smart_house_error::SmartHouseError, smart_tool::SmartTool,
    smart_tool_room::SmartToolRoom,
};
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

    ///Добавляет комнату в дом под уникальным именем
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

    pub fn remove(&mut self, name: &'static str) -> Option<SmartToolRoom> {
        self.rooms.remove(name)
    }

    pub fn get_smart_tool(
        &self,
        room_name: &'static str,
        tool_name: &'static str,
    ) -> Result<&SmartTool, SmartHouseError> {
        let room = self
            .get(room_name)
            .ok_or(SmartHouseError::RoomNotFound(room_name))?;

        let tool = room
            .get(tool_name)
            .ok_or(SmartHouseError::ToolNotFound(tool_name))?;

        Ok(tool)
    }
}

impl Report for SmartHouse {}

#[cfg(test)]
mod tests {
    use crate::{
        electro_socket::ElectroSocket, smart_house::SmartHouse, smart_house_error::SmartHouseError,
        smart_room, smart_tool::SmartTool, smart_tool_room::SmartToolRoom,
        term_detector::TermDetector,
    };
    use std::{assert_matches, panic};

    fn setup() -> SmartHouse {
        let mut house = SmartHouse::new();
        house.insert(
            "first",
            smart_room!("a1" => TermDetector::new(),
                "a2" => ElectroSocket::new(false),
                "a3" => ElectroSocket::new(true)
            ),
        );

        house.insert(
            "additional",
            smart_room!(
                "b1" => ElectroSocket::new(true),
                "b2" => TermDetector::new()
            ),
        );

        house
    }

    #[test]
    fn test_new() {
        let result = panic::catch_unwind(SmartHouse::new);

        assert!(result.is_ok(), "Код не должен паниковать");
        assert_eq!(0, result.unwrap().size(), "Некорректно создан объект");
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
        house.insert("new", smart_room!());
        assert_eq!(1, house.size(), "Неверно отработала вставка");
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
        assert!(r.is_none(), "Возвращен неверный элемент");
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
        assert!(r.is_none(), "Возвращен неверный элемент");
    }

    #[test]
    fn test_get_smart_tool() {
        let h = setup();
        let st = h.get_smart_tool("first", "a3");

        assert_matches!(st, Ok(SmartTool::ElectroSocket(t)) if t.is_switch_on(), "Некорректноу устройствао извлекли с get_smrt_tool");
    }
    #[test]
    fn test_get_smart_tool_room_not_found() {
        let h = setup();
        let st = h.get_smart_tool("any_room", "a3");

        assert_matches!(
            st,
            Err(SmartHouseError::RoomNotFound(_)),
            "Некорректно определили ошибку при отсутвии комнаты"
        );
    }
    #[test]
    fn test_remove() {
        let mut home = setup();
        let room = home.remove("first");
        assert_eq!(1, home.size(), "Неверно отработало удаление");
        assert_matches!(room, Some(room) if room.size() == 3, "Неверно отработало удаление");
    }

    #[test]
    fn test_get_smart_tool_tool_not_found() {
        let h = setup();
        let st = h.get_smart_tool("first", "u3");

        assert_matches!(
            st,
            Err(SmartHouseError::ToolNotFound(_)),
            "Некорректно определили ошибку при отсутвии устройства"
        );
    }
}
