#[derive(Debug)]
enum Message {
    // TODO: implement the message variant types based on their usage below
    ChangeColor(i32,i32,i32),
    Echo(String),
    Quit,
    Move(Point)
}
#[derive(Debug)]
struct Point {
    x: u8,
    y: u8
}

struct State {
    color: (u8, u8, u8),
    position: Point,
    quit: bool
}

impl State {
    fn _change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    fn _quit(&mut self) {
        self.quit = true;
    }

    fn _echo(&self, s: String) {
        println!("{}", s);
    }

    fn _move_position(&mut self, p: Point) {
        self.position = p;
    }

    fn process(&mut self, message: Message) {
        // TODO: create a match expression to process the different message variants
        println!("{:?}",message);
    }
}

mod tests {
    use super::*;
    
    pub fn test_match_message_call() {
        let mut state = State{
            quit: false,
            position: Point{ x: 0, y: 0 },
            color: (0, 0, 0)
        };
        state.process(Message::ChangeColor(255, 0, 255));
        state.process(Message::Echo(String::from("hello world")));
        state.process(Message::Move(Point{ x: 10, y: 15 }));
        state.process(Message::Quit);

        assert_eq!(state.color, (255, 0, 255));
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.quit, true);
    }

}

fn main(){
    tests::test_match_message_call();
}
