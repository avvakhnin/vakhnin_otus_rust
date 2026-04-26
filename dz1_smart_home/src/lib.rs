pub mod electro_socket;
pub mod smart_house;
pub mod smart_tool;
pub mod smart_tool_room;
pub mod term_detector;
#[cfg(test)]
mod tests {
    use crate::term_detector::TermDetector;

    #[test]
    fn get_current_temperature_greater_0() {
        let detector = TermDetector::new();

        // Проверяем несколько значений, чтобы убедиться, что все они в диапазоне
        for _ in 0..100 {
            let temp = detector.get_current_temperature();
            assert!(
                temp >= -40.0 && temp <= 40.0,
                "Temperature {} is outside the range [-40, 40]",
                temp
            );
        }
    }
}
