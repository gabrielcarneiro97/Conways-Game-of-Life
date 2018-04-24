use std::time::Duration;
use std::thread;


#[derive(PartialEq, Debug, Clone)]
pub enum State {
    Alive,
    Dead
}

#[derive(Debug, Clone)]
pub struct Coords {
    x: usize,
    y: usize
}

#[derive(Debug)]
pub struct Cell {
    state: State,
    position: Coords,
    neighboors: Vec<Coords>,
    neighboors_alive: i32
}

impl Cell {
    pub fn new(state: State, position: Coords, neighboors: Vec<Coords>) -> Cell {
        Cell {
            state,
            position,
            neighboors,
            neighboors_alive: 0
        }
    }

    pub fn find_neighboors(coord: Coords, true_size: &Coords) -> Vec<Coords> {
        let x = coord.x;
        let y = coord.y;

        let mut neighboors : Vec<Coords> = Vec::new();

        if y == 0 {
            if x == 0 {
                neighboors.push(Coords { x: x, y: y + 1});
                neighboors.push(Coords { x: x + 1, y: y + 1});
                neighboors.push(Coords { x: x + 1, y: y});
            } else if x == true_size.x - 1 {
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
        } else if y == true_size.y - 1 {
            if x == 0 {
                neighboors.push(Coords { x: x, y: y - 1});
                neighboors.push(Coords { x: x + 1, y: y - 1});
                neighboors.push(Coords { x: x + 1, y: y});
            } else if x == true_size.x - 1 {
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
            } else if x == true_size.x - 1 {
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

    pub fn change_state(&mut self) {
        match self.state {
            State::Alive => self.state = State::Dead,
            State::Dead => self.state = State::Alive
        };
    }
}

#[derive(Debug)]
pub struct Map {
    alives: Vec<Coords>,
    visible_size: Coords,
    true_size: Coords,
    offset: Coords,
    generation: usize
}

impl Map {
    pub fn new(visible_size: Coords) -> Map {
        let true_size = Coords {x: visible_size.x * 4, y: visible_size.y * 4};
        Map {
            alives: Vec::new(),
            offset: Coords { x: true_size.x/2, y: true_size.y/2 },
            true_size,
            visible_size,
            generation: 0
        }
    }


    pub fn next_tick(&mut self) {
        self.generation += 1;

        let mut next_gen : Vec<Coords> = Vec::new();

        for cell_coord in &self.alives {
            let mut cell = Cell::new(State::Alive, 
                Coords { x: cell_coord.x, y: cell_coord.y }, 
                Cell::find_neighboors(Coords { x: cell_coord.x, y: cell_coord.y }, 
                &self.true_size));

            for neighboor in cell.neighboors {
                if self.is_alive(&neighboor) {
                    cell.neighboors_alive += 1;
                } else {
                    let mut neighboor_cell = Cell::new(State::Dead, 
                        Coords { x: neighboor.x, y: neighboor.y }, 
                        Cell::find_neighboors(Coords { x: neighboor.x, y: neighboor.y }, 
                        &self.true_size));

                    let alives = self.count_neighboors_alive(&mut neighboor_cell);

                    if alives == 3 {
                        let already;
                        match &next_gen.iter().position(|next_gen_cell| next_gen_cell.x == neighboor.x && next_gen_cell.y == neighboor.y) {
                            Some(_) => already = true,
                            None => already = false
                        };
                        if !already {
                            next_gen.push(Coords {x: neighboor.x, y: neighboor.y});                            
                        }
                    }
                }
            }

            if cell.neighboors_alive == 2 || cell.neighboors_alive == 3 {
                next_gen.push(Coords {x: cell_coord.x, y: cell_coord.y});
            }

        }

        self.alives = next_gen;

    }

    pub fn count_neighboors_alive(&self, cell: &mut Cell) -> i32 {
        for neighboor in &cell.neighboors {
            if self.is_alive(&neighboor) {
                cell.neighboors_alive += 1;
            }
        }
        cell.neighboors_alive
    }

    pub fn map(&self) {
        let x_offset = self.offset.x;
        let y_offset = self.offset.y;
        let x_max = self.visible_size.x + x_offset;
        let y_max = self.visible_size.y + y_offset;

        for x in x_offset..x_max {
            for y in y_offset..y_max {
                if self.is_alive(&Coords {x, y}) {
                    print!("0");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }

    pub fn is_alive(&self, coord: &Coords) -> bool {
        let ret;
        match &self.alives.iter().position(|cell| cell.x == coord.x && cell.y == coord.y) {
            Some(_) => {ret = true;},
            None => {ret = false;}
        };
        ret
    }

    pub fn set_alive(&mut self, coords: Vec<Coords>) {
        for coord in coords {
            if !self.is_alive(&coord) {
                self.alives.push(Coords { x: coord.x + self.offset.x, y: coord.y + self.offset.y});
            }
        }
    }

    pub fn blinker() -> Vec<Coords> {
        let mut blinker : Vec<Coords> = Vec::new();

        blinker.push(Coords {x: 1, y: 2});
        blinker.push(Coords {x: 2, y: 2});
        blinker.push(Coords {x: 3, y: 2});
        
        blinker
    }

    pub fn glider() -> Vec<Coords> {
        let mut glider : Vec<Coords> = Vec::new();

        glider.push(Coords {x: 0, y: 0});
        glider.push(Coords {x: 0, y: 2});
        glider.push(Coords {x: 1, y: 1});
        glider.push(Coords {x: 1, y: 2});
        glider.push(Coords {x: 2, y: 1});

        glider
    }

    pub fn gosper_glider_gun() -> Vec<Coords> {
        let mut gosper_glider_gun : Vec<Coords> = Vec::new();

        gosper_glider_gun.push(Coords {x: 5, y: 1});
        gosper_glider_gun.push(Coords {x: 6, y: 1});
        gosper_glider_gun.push(Coords {x: 5, y: 2});
        gosper_glider_gun.push(Coords {x: 6, y: 2});
        gosper_glider_gun.push(Coords {x: 5, y: 11});
        gosper_glider_gun.push(Coords {x: 6, y: 11});
        gosper_glider_gun.push(Coords {x: 7, y: 11});
        gosper_glider_gun.push(Coords {x: 8, y: 12});
        gosper_glider_gun.push(Coords {x: 4, y: 12});
        gosper_glider_gun.push(Coords {x: 9, y: 13});
        gosper_glider_gun.push(Coords {x: 3, y: 13});        
        gosper_glider_gun.push(Coords {x: 9, y: 14});
        gosper_glider_gun.push(Coords {x: 3, y: 14});
        gosper_glider_gun.push(Coords {x: 6, y: 15});
        gosper_glider_gun.push(Coords {x: 4, y: 16});
        gosper_glider_gun.push(Coords {x: 8, y: 16});
        gosper_glider_gun.push(Coords {x: 5, y: 17});
        gosper_glider_gun.push(Coords {x: 6, y: 17});
        gosper_glider_gun.push(Coords {x: 7, y: 17});
        gosper_glider_gun.push(Coords {x: 6, y: 18});
        gosper_glider_gun.push(Coords {x: 3, y: 21});
        gosper_glider_gun.push(Coords {x: 4, y: 21});
        gosper_glider_gun.push(Coords {x: 5, y: 21});
        gosper_glider_gun.push(Coords {x: 3, y: 22});
        gosper_glider_gun.push(Coords {x: 4, y: 22});
        gosper_glider_gun.push(Coords {x: 5, y: 22});
        gosper_glider_gun.push(Coords {x: 2, y: 23});
        gosper_glider_gun.push(Coords {x: 6, y: 23});
        gosper_glider_gun.push(Coords {x: 1, y: 25});
        gosper_glider_gun.push(Coords {x: 2, y: 25});
        gosper_glider_gun.push(Coords {x: 6, y: 25});
        gosper_glider_gun.push(Coords {x: 7, y: 25});
        gosper_glider_gun.push(Coords {x: 3, y: 35});
        gosper_glider_gun.push(Coords {x: 4, y: 35});
        gosper_glider_gun.push(Coords {x: 3, y: 36});
        gosper_glider_gun.push(Coords {x: 4, y: 36});
        

        gosper_glider_gun
    }

}

fn main() {
    let mut map = Map::new(Coords {x: 30, y: 50});
    
    map.set_alive(Map::gosper_glider_gun());

    loop {
        thread::sleep(Duration::from_millis(100));
        print!("{}[2J", 27 as char);
        println!("------generation({})------", map.generation);
        map.map();
        map.next_tick();
    }
}
