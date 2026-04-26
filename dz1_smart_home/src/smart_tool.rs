use crate::{electro_socket::ElectroSocket, term_detector::TermDetector};

#[derive(Debug)]
pub enum SmartTool {
    TermDetector(TermDetector),
    ElectroSocket(ElectroSocket),
}

#[cfg(test)]
mod tests {
    use std::arch::x86_64::_mm_mask2_permutex2var_epi16;

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
            assert!(sts.len() > 0);
        }
    }
}
