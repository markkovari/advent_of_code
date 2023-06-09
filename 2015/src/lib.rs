pub mod eighteenth;
pub mod eigth;
pub mod eleventh;
pub mod fifteenth;
pub mod fifth;
pub mod first;
pub mod fourteenth;
pub mod fourth;
pub mod nineteenth;
pub mod ninth;
pub mod second;
pub mod seventeenth;
pub mod seventh;
pub mod sixteenth;
pub mod sixth;
pub mod tenth;
pub mod thirteenth;
pub mod trird;
pub mod twelfth;
pub mod twentieth;
pub mod twentyfifth;
pub mod twentyfirst;
pub mod twentyfourth;
pub mod twentysecond;
pub mod twentythird;

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
