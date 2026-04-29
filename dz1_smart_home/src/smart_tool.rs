//! Умное устройство
//! Реальная реализация может принадлежать одному из нескольких типов
use crate::{electro_socket::ElectroSocket, term_detector::TermDetector};

#[derive(Debug)]
pub enum SmartTool {
    TermDetector(TermDetector),
    ElectroSocket(ElectroSocket),
}

impl SmartTool {
    ///Выводит в стандартный вывод сообщение о состоянии устройства.
    pub fn report(&self) {
        println!("{:?}", self);
    }
}

#[cfg(test)]
mod tests {

    use std::panic;

    use crate::{
        electro_socket::ElectroSocket, smart_tool::SmartTool, term_detector::TermDetector,
    };

    #[test]
    fn test_debug() {
        let st1 = SmartTool::TermDetector(TermDetector::new("detector"));
        let st2 = SmartTool::ElectroSocket(ElectroSocket::new(false));
        let st3 = SmartTool::ElectroSocket(ElectroSocket::new(true));

        let debug_strings = [
            format!("{:?}", st1),
            format!("{:?}", st2),
            format!("{:?}", st3),
        ];

        for sts in debug_strings {
            assert!(!sts.is_empty());
        }
    }

    #[test]
    fn test_report() {
        let st1 = SmartTool::TermDetector(TermDetector::new("detector"));
        let st2 = SmartTool::ElectroSocket(ElectroSocket::new(false));
        let st3 = SmartTool::ElectroSocket(ElectroSocket::new(true));

        let debug_strings = [st1, st2, st3];

        let result = panic::catch_unwind(|| {
            for sts in debug_strings {
                sts.report();
            }
        });

        assert!(result.is_ok(), "Код не должен паниковать");
    }
}
