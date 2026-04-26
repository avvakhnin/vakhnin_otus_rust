//! Умная розетка
//! Позволяет управлять розеткой, получать статус включения и текущую мощность в диапазоне от 0 до 4 кВт
use std::fmt;

#[derive(Default)]
pub struct ElectroSocket {
    is_switch_on: bool,
}

impl ElectroSocket {
    pub fn new(is_switch_on: bool) -> Self {
        Self { is_switch_on }
    }

    /// Включена ли розетка
    pub fn is_switch_on(&self) -> bool {
        self.is_switch_on
    }

    /// Текущая мощность розетки в Вт
    fn get_power_value(&self) -> i32 {
        match self.is_switch_on {
            false => 0,
            true => rand::random_range(1..4000),
        }
    }

    ///Включаем
    fn switch_on(&mut self) {
        self.is_switch_on = true;
    }

    ///Выключаем
    fn swich_off(&mut self) {
        self.is_switch_on = false;
    }
}

impl fmt::Debug for ElectroSocket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.is_switch_on;
        let p = self.get_power_value();
        let ps = format!("{:.2}кВт", p as f32 / 1000.);
        f.debug_struct("ElectroSocket")
            .field("is_switch_on", &s)
            .field("power_value", &ps)
            .finish()
    }
}

#[cfg(test)]
mod tests {

    use std::arch::x86_64::_CMP_FALSE_OQ;

    use crate::electro_socket::ElectroSocket;

    #[test]
    fn test_default() {
        let e = ElectroSocket::default();
        assert!(
            !e.is_switch_on,
            "По умолчанию розетка должа создаваться выключенной"
        );
    }

    #[test]
    fn test_new() {
        let e1 = ElectroSocket::new(true);
        let e2 = ElectroSocket::new(false);

        assert!(
            e1.is_switch_on,
            "Неверно обработано значение переданное в конструкторе"
        );
        assert!(
            !e2.is_switch_on,
            "Неверно обработано значение переданное в конструкторе"
        )
    }

    #[test]
    fn test_is_switch_on() {
        let e1 = ElectroSocket::new(false);
        let e2 = ElectroSocket::new(true);
        assert!(
            !e1.is_switch_on(),
            "Не правильно отрабатывает диагностика статуса включения"
        );
        assert!(
            e2.is_switch_on(),
            "Не правильно отрабатывает диагностика статуса включения"
        );
    }
    #[test]
    fn test_switch_on() {
        let mut e = ElectroSocket::new(false);
        e.switch_on();
        assert!(e.is_switch_on, "Не удалось включить розетку");
    }

    #[test]
    fn test_switch_off() {
        let mut e = ElectroSocket::new(true);
        e.swich_off();
        assert!(!e.is_switch_on, "Не удалось выключить розетку");
    }

    #[test]
    fn test_get_power_value() {
        let e1 = ElectroSocket::new(true);
        let e2 = ElectroSocket::new(false);

        assert_ne!(0, e1.get_power_value(), "Неправильное значение мощности");
        assert_eq!(0, e2.get_power_value(), "Неправильное значение мощности");
    }

    #[test]
    fn test_get_power_value_range() {
        let e = ElectroSocket::new(true);
        for _ in 0..100 {
            assert!(e.get_power_value() > 0, "Напряжение слишком низкое");
            assert!(e.get_power_value() < 4000, "Напряжение слишком высокое");
        }
    }

    #[test]
    fn test_debug() {
        let e = ElectroSocket::new(true);
        let debug_str = format!("{:?}", e);
        assert!(
            debug_str.starts_with("ElectroSocket { is_switch_on: true, power_value: \""),
            "Неверный формат вывода"
        );
        assert!(debug_str.ends_with("кВт\" }"), "Неверный формат вывода");
    }
}
