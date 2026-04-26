use crate::smart_tool::SmartTool;
use std::fmt;
#[derive(Debug)]
pub struct SmartToolRoom {
    smart_tools: Vec<SmartTool>,
}

impl SmartToolRoom {
    pub fn new(smart_tools: Vec<SmartTool>) -> SmartToolRoom {
        SmartToolRoom { smart_tools }
    }

    pub fn print(&self) {
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

    #[test]
    fn test_print() {
        let st1 = SmartTool::TermDetector(TermDetector::new());
        let st2 = SmartTool::ElectroSocket(ElectroSocket::new(false));
        let st3 = SmartTool::ElectroSocket(ElectroSocket::new(true));
        let r = SmartToolRoom::new(vec![st1, st2, st3]);

        let result = panic::catch_unwind(|| {
            r.print();
        });

        assert!(result.is_ok(), "Код не должен паниковать");
    }
}
