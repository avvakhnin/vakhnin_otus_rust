use crate::smart_tool_room::SmartToolRoom;
#[derive(Debug)]
pub struct SmartHouse {
    rooms: Vec<SmartToolRoom>,
}

impl SmartHouse {
    pub fn new(rooms: Vec<SmartToolRoom>) -> Self {
        SmartHouse { rooms }
    }

    pub fn size(&self) -> usize {
        self.rooms.len()
    }

    pub fn get(&self, ix: usize) -> &SmartToolRoom {
        &self.rooms[ix]
    }

    pub fn get_mut(&mut self, ix: usize) -> &mut SmartToolRoom {
        &mut self.rooms[ix]
    }

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
    use std::panic;

    fn setup() -> SmartHouse {
        let st1 = SmartTool::TermDetector(TermDetector::new());
        let st2 = SmartTool::ElectroSocket(ElectroSocket::new(false));
        let st3 = SmartTool::ElectroSocket(ElectroSocket::new(true));
        let str1 = SmartToolRoom::new(vec![st1, st2, st3]);

        let st1 = SmartTool::ElectroSocket(ElectroSocket::new(true));
        let st2 = SmartTool::TermDetector(TermDetector::new());
        let str2 = SmartToolRoom::new(vec![st1, st2]);

        SmartHouse::new(vec![str1, str2])
    }

    #[test]
    fn test_new() {
        let result = panic::catch_unwind(|| {
            setup();
        });

        assert!(result.is_ok(), "Код не должен паниковать");
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
    fn test_get() {
        let h = setup();
        let r = h.get(1);
        assert_eq!(2, r.size(), "Возвращен неверный элемент");
    }

    #[test]
    #[should_panic]
    fn test_get_panic() {
        let h = setup();
        h.get(100);
    }

    #[test]
    fn test_get_mut() {
        let mut h = setup();
        let r = h.get_mut(0);
        assert_eq!(3, r.size(), "Возвращен неверный элемент");
    }

    #[test]
    #[should_panic]
    fn test_get_mut_panix() {
        let mut h = setup();
        h.get_mut(100);
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
