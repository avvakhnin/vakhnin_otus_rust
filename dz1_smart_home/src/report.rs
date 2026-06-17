use std::fmt::Debug;

pub trait Report: Debug {
    fn get_report(&self) -> String {
        format!("{:?}", self)
    }
    fn get_report_pretty(&self) -> String {
        format!("{:#?}", self)
    }
}
