use dz1_smart_home::term_detector::TermDetector;

fn main() {
    let t = TermDetector::new();
    println!("Hello, world! {}", t.get_current_temperature());
}
