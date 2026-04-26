use crate::smart_tool_room::SmartToolRoom;

pub struct SmartHouse {
    rooms: Vec<SmartToolRoom>,
}

impl SmartHouse {
    pub fn new(rooms: Vec<SmartToolRoom>) -> Self {
        SmartHouse { rooms }
    }
}
