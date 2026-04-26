use crate::smart_tool::SmartTool;
#[derive(Debug)]
pub struct SmartToolRoom {
    tools: Vec<SmartTool>,
}

impl SmartToolRoom {
    pub fn new(tools: Vec<SmartTool>) -> SmartToolRoom {
        SmartToolRoom { tools }
    }
}
