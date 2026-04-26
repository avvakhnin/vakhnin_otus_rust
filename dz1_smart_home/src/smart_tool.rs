use crate::{electro_socket::ElectroSocket, term_detector::TermDetector};

#[derive(Debug)]
pub enum SmartTool {
    TermDetector(TermDetector),
    ElectroSocket(ElectroSocket),
}

impl SmartTool {
    pub fn print(&self) {
        println!("{:?}", self);
    }
}

#[cfg(test)]
mod tests {

    use std::{panic, ptr::copy_nonoverlapping};

    use crate::{
        electro_socket::ElectroSocket, smart_tool::SmartTool, term_detector::TermDetector,
    };

    #[test]
    fn test_debug() {
        let st1 = SmartTool::TermDetector(TermDetector::new());
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
    fn test_print() {
        let st1 = SmartTool::TermDetector(TermDetector::new());
        let st2 = SmartTool::ElectroSocket(ElectroSocket::new(false));
        let st3 = SmartTool::ElectroSocket(ElectroSocket::new(true));

        let debug_strings = [st1, st2, st3];

        let result = panic::catch_unwind(|| {
            for sts in debug_strings {
                sts.print();
            }
        });

        assert!(result.is_ok(), "Код не должен паниковать");
    }
}
