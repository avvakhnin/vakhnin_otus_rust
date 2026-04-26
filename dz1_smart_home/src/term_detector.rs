//! Умный термометр
//! Выдаёт температуру в диапазоне -40°C до 40°Cy
//!
use std::fmt;
pub struct TermDetector {}

impl TermDetector {
    pub fn new() -> Self {
        Self {}
    }

    /// Выдаёт температуру в диапазоне -40°C до 40°C
    pub fn get_current_temperature(&self) -> f32 {
        rand::random_range(-40f32..40f32)
    }
}

impl fmt::Debug for TermDetector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let t = self.get_current_temperature();
        let ts = format!("{:.2}°C", t);
        f.debug_struct("TermDetector")
            .field("temperature_celsius", &ts)
            .finish()
    }
}
#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use crate::term_detector::TermDetector;

    #[test]
    fn test_value_in_range() {
        let td = TermDetector::new();
        for _ in 0..100 {
            let t = td.get_current_temperature();
            assert!(t > -40., "Показатели температуры {} меньше допустимых", t);
            assert!(t < 40., "Показатели температуры {} выше допустимых", t);
        }
    }

    #[test]
    fn test_debug() {
        let td = TermDetector::new();
        let debug_str = format!("{:?}", td);
        assert!(
            debug_str.starts_with("TermDetector { temperature_celsius: \""),
            "Неверный формат вывода"
        );
        assert!(debug_str.ends_with("°C\" }"), "Неверный формат вывода");
    }
}
