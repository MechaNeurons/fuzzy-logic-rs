use fuzzy_logic_rs::memberships::{Gaussian, GetDegree};
fn main() {
    let g = Gaussian::new(1.0, 3.0);
    let h = g.get_degree(1.0);
    println!("{h}");
}
