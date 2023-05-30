pub mod eigth;
pub mod fifth;
pub mod first;
pub mod fourth;
pub mod second;
pub mod seventh;
pub mod sixth;
pub mod trird;
pub mod ninth;
pub mod tenth;

struct Excercise {
    content: String,
    example: String,
}

trait Readable {
    fn get_example(&self) -> String;
    fn get_prod(&self) -> String;
}

impl Readable for Excercise {
    fn get_example(&self) -> String {
        self.example.clone()
    }
    fn get_prod(&self) -> String {
        self.content.clone()
    }
}

trait Solvable {
    fn first(&self, content: String) -> i32;
    fn solve_first(&self, is_prod: bool) -> i32;
    fn second(&self, content: String) -> i32;
    fn solve_second(&self, is_prod: bool) -> i32;
}
