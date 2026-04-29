//! Умный термометр
//! Выдаёт температуру в диапазоне -40°C до 40°Cy
//!
use std::fmt;
pub struct TermDetector {
    name: String,
}

impl TermDetector {
    ///Конструктор по-умолчанию
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
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
        f.debug_struct(&format!("TermDetector \"{}\"", self.get_name()))
            .field("temperature_celsius", &ts)
            .finish()
    }
}

#[cfg(test)]
mod tests {

    use crate::term_detector::TermDetector;

    #[test]
    fn test_name() {
        let td = TermDetector::new("special 1");
        assert_eq!(
            "special 1",
            td.get_name(),
            "Вернулось некорректное название датчика"
        );
    }

    #[test]
    fn test_value_in_range() {
        let td = TermDetector::new("any");

        for _ in 0..100 {
            let t = td.get_current_temperature();
            assert!(t > -40., "Показатели температуры {} меньше допустимых", t);
            assert!(t < 40., "Показатели температуры {} выше допустимых", t);
        }
    }

    #[test]
    fn test_debug() {
        let td = TermDetector::new("any");
        let debug_str = format!("{:?}", td);
        assert!(
            debug_str.starts_with("TermDetector \"any\" { temperature_celsius: \""),
            "Неверный формат вывода"
        );
        assert!(debug_str.ends_with("°C\" }"), "Неверный формат вывода");
    }
}
