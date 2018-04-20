#[derive(Debug)]
enum State {
    Alive,
    Dead
}

struct Position {
    x: usize,
    y: usize
}

struct Cell {
    state: State,
    position: Position
}

fn main() {
    let a = State::Alive;
    println!("{:?}", a);
}
