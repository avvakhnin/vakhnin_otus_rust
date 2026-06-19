use std::{error::Error, fmt};

#[derive(Debug)]
pub enum SmartHouseError {
    ToolNotFound(&'static str),
    RoomNotFound(&'static str),
}

impl fmt::Display for SmartHouseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SmartHouseError::ToolNotFound(name) => {
                write!(f, "Tool '{}' not found", name)
            }

            SmartHouseError::RoomNotFound(name) => {
                write!(f, "Room '{}' not found", name)
            }
        }
    }
}

impl Error for SmartHouseError {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn test_room_not_found_display() {
        let error = SmartHouseError::RoomNotFound("kitchen");
        assert_eq!(error.to_string(), "Room 'kitchen' not found");
    }

    #[test]
    fn test_tool_not_found_display() {
        let error = SmartHouseError::ToolNotFound("micro");
        assert_eq!(error.to_string(), "Tool 'micro' not found");
    }

    #[test]
    fn test_error_trait_implemented() {
        // Проверяем, что тип действительно реализует Error
        fn is_error<T: Error>() -> bool {
            true
        }
        assert!(is_error::<SmartHouseError>());
    }

    #[test]
    fn test_error_debug_format() {
        let error = SmartHouseError::RoomNotFound("kitchen");
        let debug_str = format!("{:?}", error);
        // Проверяем, что Debug включает все варианты
        assert!(debug_str.contains("RoomNotFound"));
        assert!(debug_str.contains("kitchen"));
    }
}
