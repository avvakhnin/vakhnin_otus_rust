use crate::smart_tool::SmartTool;
pub struct SmartToolRoom {
    tools: Vec<SmartTool>,
}

impl SmartToolRoom {
    pub fn new(tools: Vec<SmartTool>) -> SmartToolRoom {
        SmartToolRoom { tools }
    }
}
