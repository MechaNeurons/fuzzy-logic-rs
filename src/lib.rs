pub mod memberships;

#[allow(unused)]
use memberships::{Triangle, Universe};

#[cfg(test)]
#[allow(dead_code, unused)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let u1: Universe<50> = Universe::new(0.0, 5.0);
        let tr1 = Triangle::new(&u1, 1.0, 2.0, 3.0);
        println!("{:?}", tr1);
    }
}
