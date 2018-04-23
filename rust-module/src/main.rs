use std::rc::Rc;
use std::cell::RefCell;
use std::time::Duration;
use std::thread;


#[derive(PartialEq, Debug, Clone)]
enum State {
    Alive,
    Dead
}

#[derive(Debug, Clone)]
struct Coords {
    x: usize,
    y: usize
}

#[derive(Debug)]
struct Cell {
    state: State,
    position: Coords,
    neighboors: Vec<Coords>,
    neighboors_alive: i32
}

impl Cell {
    fn new(state: State, position: Coords, neighboors: Vec<Coords>) -> Cell {
        Cell {
            state,
            position,
            neighboors,
            neighboors_alive: 0
        }
    }

    fn find_neighboors(coord: Coords, size: &Coords) -> Vec<Coords> {
        let x = coord.x;
        let y = coord.y;

        let mut neighboors : Vec<Coords> = Vec::new();

        if y == 0 {
            if x == 0 {
                neighboors.push(Coords { x: x, y: y + 1});
                neighboors.push(Coords { x: x + 1, y: y + 1});
                neighboors.push(Coords { x: x + 1, y: y});
            } else if x == size.x - 1 {
                neighboors.push(Coords { x: x - 1, y: y});
                neighboors.push(Coords { x: x - 1, y: y + 1});
                neighboors.push(Coords { x: x, y: y + 1});
            } else {
                neighboors.push(Coords { x: x - 1, y: y});
                neighboors.push(Coords { x: x - 1, y: y + 1});
                neighboors.push(Coords { x: x, y: y + 1});
                neighboors.push(Coords { x: x + 1, y: y + 1});
                neighboors.push(Coords { x: x + 1, y: y});
            }
        } else if y == size.y - 1 {
            if x == 0 {
                neighboors.push(Coords { x: x, y: y - 1});
                neighboors.push(Coords { x: x + 1, y: y - 1});
                neighboors.push(Coords { x: x + 1, y: y});
            } else if x == size.x - 1 {
                neighboors.push(Coords { x: x - 1, y: y});
                neighboors.push(Coords { x: x - 1, y: y - 1});
                neighboors.push(Coords { x: x, y: y - 1});
            } else {
                neighboors.push(Coords { x: x - 1, y: y});
                neighboors.push(Coords { x: x - 1, y: y - 1});
                neighboors.push(Coords { x: x, y: y - 1});
                neighboors.push(Coords { x: x + 1, y: y - 1});
                neighboors.push(Coords { x: x + 1, y: y});
            }
        } else {
            if x == 0 {
                neighboors.push(Coords { x: x, y: y - 1});
                neighboors.push(Coords { x: x + 1, y: y - 1});
                neighboors.push(Coords { x: x + 1, y: y});
                neighboors.push(Coords { x: x + 1, y: y + 1});
                neighboors.push(Coords { x: x, y: y + 1});
            } else if x == size.x - 1 {
                neighboors.push(Coords { x: x, y: y - 1});
                neighboors.push(Coords { x: x - 1, y: y - 1});
                neighboors.push(Coords { x: x - 1, y: y});
                neighboors.push(Coords { x: x - 1, y: y + 1});
                neighboors.push(Coords { x: x, y: y + 1});
            } else {
                neighboors.push(Coords { x: x - 1, y: y - 1});
                neighboors.push(Coords { x: x - 1, y: y});
                neighboors.push(Coords { x: x - 1, y: y + 1});
                neighboors.push(Coords { x: x, y: y + 1});
                neighboors.push(Coords { x: x + 1, y: y + 1});
                neighboors.push(Coords { x: x + 1, y: y});
                neighboors.push(Coords { x: x + 1, y: y - 1});
                neighboors.push(Coords { x: x, y: y - 1});
            }
        }

        neighboors
    }

    fn change_state(&mut self) {
        match self.state {
            State::Alive => self.state = State::Dead,
            State::Dead => self.state = State::Alive
        };
    }
}

#[derive(Debug)]
struct Map {
    cells: Vec<Rc<RefCell<Cell>>>,
    size: Coords,
    counter: usize
}

impl Map {
    fn new(size: Coords) -> Map {
        Map {
            cells: Map::populate(&size),
            size,
            counter: 0
        }
    }

    fn populate(size: &Coords) -> Vec<Rc<RefCell<Cell>>> {

        let mut vec : Vec<Rc<RefCell<Cell>>> = Vec::new();

        for x in 0..size.x {
            for y in 0..size.y {
                let cell = Cell::new(State::Dead, Coords { x, y }, Cell::find_neighboors(Coords { x, y }, &size));
                vec.push(Rc::new(RefCell::new(cell)));
            }
        }

        vec
    }

    fn next_tick(&mut self) {
        self.counter += 1;

        for cell in &self.cells {
            let mut borrow = cell.borrow_mut();
            let mut alives = 0;

            for neighboor in &borrow.neighboors {
                let neighboor_cell = self.get_cell(&neighboor);
                let state = &neighboor_cell.borrow().state;

                match *state {
                    State::Alive => alives += 1,
                    State::Dead => ()
                }
            }
            borrow.neighboors_alive = alives;            
        }

        for cell in &self.cells {
            let mut borrow = cell.borrow_mut();
            let alives = borrow.neighboors_alive;

            let state = borrow.state.clone();

            match state {
                State::Alive => {
                    if alives < 2 || alives > 3 {
                        borrow.change_state();
                    }
                },
                State::Dead => {
                    if alives == 3 {
                        borrow.change_state();
                    }
                }
            }
        }

    }

    fn map(&self) {
        let x_max = self.size.x;
        let y_max = self.size.y;

        for x in 0..x_max {
            for y in 0..y_max {
                let cell = self.get_cell(&Coords {x: x, y: y});
                let state = &cell.borrow().state;

                match *state {
                    State::Alive => print!("▀"),
                    State::Dead => print!("╳")
                }
            }
            println!("");
        }
    }

    fn get_cell(&self, coord: &Coords) -> Rc<RefCell<Cell>> {
        let pos = coord.y + (coord.x * &self.size.y);

        Rc::clone(&self.cells[pos])
    }

    fn set_moc(&self) {
        let cell = self.get_cell(&Coords {x: 1, y: 3});
        cell.borrow_mut().change_state();
        let cell = self.get_cell(&Coords {x: 2, y: 1});
        cell.borrow_mut().change_state();
        let cell = self.get_cell(&Coords {x: 2, y: 4});
        cell.borrow_mut().change_state();
        let cell = self.get_cell(&Coords {x: 3, y: 1});
        cell.borrow_mut().change_state();
        let cell = self.get_cell(&Coords {x: 3, y: 4});
        cell.borrow_mut().change_state();
        let cell = self.get_cell(&Coords {x: 4, y: 2});
        cell.borrow_mut().change_state();
    }

    fn set(&self, coords: Vec<Coords>) {
        for coord in coords {
            let cell = self.get_cell(&coord);
            cell.borrow_mut().state = State::Alive;
        }
    }

}

fn main() {
    let mut map = Map::new(Coords {x: 6, y: 6});
    let mut vec : Vec<Coords> = Vec::new();

    vec.push(Coords {x: 1, y: 3});
    vec.push(Coords {x: 2, y: 1});
    vec.push(Coords {x: 2, y: 4});
    vec.push(Coords {x: 3, y: 1});
    vec.push(Coords {x: 3, y: 4});
    vec.push(Coords {x: 4, y: 2});

    map.set(vec);

    loop {
        thread::sleep(Duration::from_millis(1000));
        print!("{}[2J", 27 as char);
        map.map();
        map.next_tick();
    }
}
