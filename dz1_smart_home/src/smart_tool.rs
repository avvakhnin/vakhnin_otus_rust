//! Умное устройство
//! Реальная реализация может принадлежать одному из нескольких типов
use crate::{electro_socket::ElectroSocket, report::Report, term_detector::TermDetector};

#[derive(Debug)]
pub enum SmartTool {
    TermDetector(TermDetector),
    ElectroSocket(ElectroSocket),
}

impl Report for SmartTool {}

impl From<TermDetector> for SmartTool {
    fn from(value: TermDetector) -> Self {
        SmartTool::TermDetector(value)
    }
}

impl From<ElectroSocket> for SmartTool {
    fn from(value: ElectroSocket) -> Self {
        SmartTool::ElectroSocket(value)
    }
}

#[cfg(test)]
mod tests {

    use std::assert_matches;

    use crate::{
        electro_socket::ElectroSocket, smart_tool::SmartTool, term_detector::TermDetector,
    };

    #[test]
    fn test_debug() {
        let st1 = SmartTool::TermDetector(TermDetector::default());
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
    fn test_from_detector() {
        let st = SmartTool::from(TermDetector::default());
        assert_matches!(st, SmartTool::TermDetector(_));
    }

    #[test]
    fn test_from_electro_socket() {
        let st = SmartTool::from(ElectroSocket::new(true));
        assert_matches!(st, SmartTool::ElectroSocket(t) if t.is_switch_on(), "Некорректная реализация From<T>");
    }
}
