pub struct ElectroSocket {
    is_switch_on: bool,
}

impl ElectroSocket {
    fn new(is_switch_on: bool) -> Self {
        Self { is_switch_on }
    }

    pub fn is_switch_on(&self) -> bool {
        self.is_switch_on
    }

    fn switch_on(&mut self) {
        self.is_switch_on = true;
    }

    fn swich_off(&mut self) {
        self.is_switch_on = false;
    }
}
