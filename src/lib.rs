pub mod memberships;

#[allow(unused)]
use memberships::{MemberShip, Universe};

#[cfg(test)]
#[allow(dead_code, unused)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let u1: Universe<100> = Universe::new(0.0, 11.0);
        let mf1 = MemberShip::new_triangle(&u1, 1.0, 2.0, 3.0);
        let mf2 = MemberShip::new_triangle(&u1, 2.5, 5.0, 7.5);
        println!("{:?}",mf1+mf2);
    }
}
