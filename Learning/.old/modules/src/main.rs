mod test {
    pub mod helper {
        pub fn test(x: &str) {
            println!("{}", x)
        }
    }
}

mod advanced;
use crate::advanced::helper;

fn main() {
    test::helper::test("Hello!");
    helper::test("Hello v2!");
}
