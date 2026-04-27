//! Комната, содержащая массив умных устройств
use crate::smart_tool::SmartTool;
#[derive(Debug)]
pub struct SmartToolRoom {
    smart_tools: Vec<SmartTool>,
}

impl SmartToolRoom {
    ///Создает комнату со списком умных устройств
    pub fn new(smart_tools: Vec<SmartTool>) -> SmartToolRoom {
        SmartToolRoom { smart_tools }
    }

    ///Возвращает количество устройств в комнате
    pub fn size(&self) -> usize {
        self.smart_tools.len()
    }
    ///Возвращает ссылку на устройство по указанному индексу
    pub fn get(&self, ix: usize) -> &SmartTool {
        &self.smart_tools[ix]
    }

    ///Возвращает мутабельную ссылку на устройство по указанному индексу.
    pub fn get_mut(&mut self, ix: usize) -> &mut SmartTool {
        &mut self.smart_tools[ix]
    }
    /// Выводит в стандартный вывод отчёт о всех устройствах в комнате.
    pub fn report(&self) {
        println!("{:?}", self);
    }
}
#[cfg(test)]
mod tests {
    use crate::{
        electro_socket::ElectroSocket, smart_tool::SmartTool, smart_tool_room::SmartToolRoom,
        term_detector::TermDetector,
    };
    use std::panic;

    fn setup() -> SmartToolRoom {
        let st1 = SmartTool::TermDetector(TermDetector::new());
        let st2 = SmartTool::ElectroSocket(ElectroSocket::new(false));
        let st3 = SmartTool::ElectroSocket(ElectroSocket::new(true));
        SmartToolRoom::new(vec![st1, st2, st3])
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
        let r = setup();
        assert_eq!(
            3,
            r.size(),
            "Некорректно определёно количеставо датчиков в комнате"
        );
    }

    #[test]
    fn test_get() {
        let r = setup();
        let t = r.get(1);
        assert!(
            matches!(t, SmartTool::ElectroSocket { .. }),
            "Возвращен неверный элемент"
        );
        if let SmartTool::ElectroSocket(e) = t {
            assert!(!e.is_switch_on(), "Возвращен неверный элемент");
        }
    }

    #[test]
    #[should_panic]
    fn test_get_panic() {
        let r = setup();
        r.get(100);
    }

    #[test]
    fn test_get_mut() {
        let mut r = setup();
        let mut t = r.get_mut(1);
        assert!(
            matches!(t, SmartTool::ElectroSocket { .. }),
            "Возвращен неверный элемент"
        );
        if let SmartTool::ElectroSocket(e) = t {
            assert!(!e.is_switch_on(), "Возвращен неверный элемент");
            e.switch_on();
            assert!(e.is_switch_on(), "Возвращен неверный элемент");
        }
    }

    #[test]
    #[should_panic]
    fn test_get_mut_panic() {
        let mut r = setup();
        r.get_mut(100);
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
