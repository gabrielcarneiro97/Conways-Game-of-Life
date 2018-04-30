#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

extern crate bit_vec;

use bit_vec::BitVec;

#[derive(PartialEq, Debug, Clone)]
#[wasm_bindgen]
pub enum State {
    Alive,
    Dead
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct Coords {
    x: i32,
    y: i32
}

#[derive(Debug)]
#[wasm_bindgen]
pub struct Cell {
    state: State,
    position: i32,
    neighboors: Vec<i32>,
    neighboors_alive: i32
}

#[wasm_bindgen]
impl Cell {
    pub fn new(state: State, position: i32, neighboors: Vec<i32>) -> Cell {
        Cell {
            state,
            position,
            neighboors,
            neighboors_alive: 0
        }
    }

    pub fn change_state(&mut self) {
        match self.state {
            State::Alive => self.state = State::Dead,
            State::Dead => self.state = State::Alive
        };
    }
}

#[derive(Debug)]
#[wasm_bindgen]
pub struct Map {
    alives: Vec<i32>,
    bit_map: BitVec,
    visible_size: Coords,
    true_size: Coords,
    offset: Coords,
    generation: i32
}

#[wasm_bindgen]
impl Map {
    pub fn new(visible_size: Coords) -> Map {
        let true_size = Coords {x: visible_size.x * 4, y: visible_size.y * 4};
        Map {
            alives: Vec::new(),
            bit_map: BitVec::from_elem((true_size.x * true_size.y) as usize, false),
            offset: Coords { x: true_size.x/2, y: true_size.y/2 },
            true_size,
            visible_size,
            generation: 0
        }
    }

    pub fn get_alives(&self) -> Vec<i32> {
        self.alives.clone()
    }

    pub fn next_tick(&mut self) {
        self.generation += 1;

        let mut next_gen : Vec<i32> = Vec::new();
        let mut kill : Vec<i32> = Vec::new();

        for cell_bit_pos in &self.alives {
            let cell_coord = self.get_coords(*cell_bit_pos);
            let mut cell = Cell::new(State::Alive, 
                *cell_bit_pos, 
                self.find_neighboors(*cell_bit_pos));

            for neighboor in cell.neighboors {
                if self.is_alive(neighboor) {
                    cell.neighboors_alive += 1;
                } else {
                    let mut neighboor_cell = Cell::new(State::Dead, 
                        neighboor, 
                        self.find_neighboors(neighboor));

                    let alives = self.count_neighboors_alive(&mut neighboor_cell);

                    if alives == 3 {
                        let already;
                        match &next_gen.iter().position(|next_gen_cell| next_gen_cell == &neighboor) {
                            Some(_) => already = true,
                            None => already = false
                        };
                        if !already {
                            next_gen.push(neighboor);                            
                        }
                    }
                }
            }

            if cell.neighboors_alive == 2 || cell.neighboors_alive == 3 {
                next_gen.push(self.get_pos(&Coords {x: cell_coord.x, y: cell_coord.y}));
            } else {
                kill.push(self.get_pos(&cell_coord));
            }

        }

        for pos in &next_gen {
            self.bit_map.set(*pos as usize, true);
        }

        for to_kill in kill {
            self.bit_map.set(to_kill as usize, false);
        }

        self.alives = next_gen;

    }

        pub fn find_neighboors(&self, pos: i32) -> Vec<i32> {
        
        let coord = self.get_coords(pos);
        let x = coord.x;
        let y = coord.y;

        let mut neighboors : Vec<i32> = Vec::new();

        if y == 0 {
            if x == 0 {
                neighboors.push(self.get_pos(&Coords { x: x, y: y + 1}));
                neighboors.push(self.get_pos(&Coords { x: x + 1, y: y + 1}));
                neighboors.push(self.get_pos(&Coords { x: x + 1, y: y}));
            } else if x == self.true_size.x - 1 {
                neighboors.push(self.get_pos(&Coords { x: x - 1, y: y}));
                neighboors.push(self.get_pos(&Coords { x: x - 1, y: y + 1}));
                neighboors.push(self.get_pos(&Coords { x: x, y: y + 1}));
            } else {
                neighboors.push(self.get_pos(&Coords { x: x - 1, y: y}));
                neighboors.push(self.get_pos(&Coords { x: x - 1, y: y + 1}));
                neighboors.push(self.get_pos(&Coords { x: x, y: y + 1}));
                neighboors.push(self.get_pos(&Coords { x: x + 1, y: y + 1}));
                neighboors.push(self.get_pos(&Coords { x: x + 1, y: y}));
            }
        } else if y == self.true_size.y - 1 {
            if x == 0 {
                neighboors.push(self.get_pos(&Coords { x: x, y: y - 1}));
                neighboors.push(self.get_pos(&Coords { x: x + 1, y: y - 1}));
                neighboors.push(self.get_pos(&Coords { x: x + 1, y: y}));
            } else if x == self.true_size.x - 1 {
                neighboors.push(self.get_pos(&Coords { x: x - 1, y: y}));
                neighboors.push(self.get_pos(&Coords { x: x - 1, y: y - 1}));
                neighboors.push(self.get_pos(&Coords { x: x, y: y - 1}));
            } else {
                neighboors.push(self.get_pos(&Coords { x: x - 1, y: y}));
                neighboors.push(self.get_pos(&Coords { x: x - 1, y: y - 1}));
                neighboors.push(self.get_pos(&Coords { x: x, y: y - 1}));
                neighboors.push(self.get_pos(&Coords { x: x + 1, y: y - 1}));
                neighboors.push(self.get_pos(&Coords { x: x + 1, y: y}));
            }
        } else {
            if x == 0 {
                neighboors.push(self.get_pos(&Coords { x: x, y: y - 1}));
                neighboors.push(self.get_pos(&Coords { x: x + 1, y: y - 1}));
                neighboors.push(self.get_pos(&Coords { x: x + 1, y: y}));
                neighboors.push(self.get_pos(&Coords { x: x + 1, y: y + 1}));
                neighboors.push(self.get_pos(&Coords { x: x, y: y + 1}));
            } else if x == self.true_size.x - 1 {
                neighboors.push(self.get_pos(&Coords { x: x, y: y - 1}));
                neighboors.push(self.get_pos(&Coords { x: x - 1, y: y - 1}));
                neighboors.push(self.get_pos(&Coords { x: x - 1, y: y}));
                neighboors.push(self.get_pos(&Coords { x: x - 1, y: y + 1}));
                neighboors.push(self.get_pos(&Coords { x: x, y: y + 1}));
            } else {
                neighboors.push(self.get_pos(&Coords { x: x - 1, y: y - 1}));
                neighboors.push(self.get_pos(&Coords { x: x - 1, y: y}));
                neighboors.push(self.get_pos(&Coords { x: x - 1, y: y + 1}));
                neighboors.push(self.get_pos(&Coords { x: x, y: y + 1}));
                neighboors.push(self.get_pos(&Coords { x: x + 1, y: y + 1}));
                neighboors.push(self.get_pos(&Coords { x: x + 1, y: y}));
                neighboors.push(self.get_pos(&Coords { x: x + 1, y: y - 1}));
                neighboors.push(self.get_pos(&Coords { x: x, y: y - 1}));
            }
        }

        neighboors
    }

    pub fn count_neighboors_alive(&self, cell: &mut Cell) -> i32 {
        for neighboor in &cell.neighboors {
            if self.is_alive(*neighboor) {
                cell.neighboors_alive += 1;
            }
        }
        cell.neighboors_alive
    }

    pub fn get_map(&self) -> Vec<i32> {
        let x_offset = self.offset.x;
        let y_offset = self.offset.y;
        let x_max = self.visible_size.x + x_offset;
        let y_max = self.visible_size.y + y_offset;

        let mut vec : Vec<i32> = Vec::new();

        for x in x_offset..x_max {
            for y in y_offset..y_max {
                if self.is_alive(self.get_pos(&Coords {x, y})) {
                    vec.push(1);
                } else {
                    vec.push(0);
                }
            }
        }

        vec
    }

    pub fn get_map_alives(&self) -> Vec<i32> {

        let x_offset = self.offset.x;
        let y_offset = self.offset.y;
        let x_max = self.visible_size.x + x_offset;
        let y_max = self.visible_size.y + y_offset;

        let mut vec : Vec<i32> = Vec::new();

        for x in x_offset..x_max {
            for y in y_offset..y_max {
                if self.is_alive(self.get_pos(&Coords {x, y})) {
                    vec.push((y - y_offset) + ((x - x_offset) * self.visible_size.y));
                }
            }
        }
        vec
    }

    pub fn get_pos(&self, coord: &Coords) -> i32 {
        coord.y + (coord.x * self.true_size.y)
    }

    pub fn get_coords(&self, pos: i32) -> Coords {
        Coords {
            x: pos / self.true_size.y,
            y: pos % self.true_size.y
        }
    }

    pub fn is_alive(&self, pos: i32) -> bool {
        self.bit_map[pos as usize]
    }

    pub fn set_alive(&mut self, pos_vec: Vec<i32>) {
        for pos in pos_vec {
            if !self.is_alive(pos) {
                self.alives.push(pos);
                self.bit_map.set(pos as usize, true);
            }
        }
    }
    
    pub fn offset_pos(&self, coords: Coords) -> i32 {
        self.get_pos(&Coords {
            x: coords.x + self.offset.x,
            y: coords.y + self.offset.y
        })
    }

    pub fn blinker(&self) -> Vec<i32> {
        let mut blinker : Vec<i32> = Vec::new();

        blinker.push(self.offset_pos(Coords {x: 0, y: 1}));
        blinker.push(self.offset_pos(Coords {x: 1, y: 1}));
        blinker.push(self.offset_pos(Coords {x: 2, y: 1}));
        
        blinker
    }

    pub fn party(&self) -> Vec<i32> {
        let mut party : Vec<i32> = Vec::new();

        party.push(self.offset_pos(Coords {x: 3, y: 6}));
        party.push(self.offset_pos(Coords {x: 1, y: 12}));
        party.push(self.offset_pos(Coords {x: 3, y: 2}));
        party.push(self.offset_pos(Coords {x: 8, y: 1}));
        party.push(self.offset_pos(Coords {x: 3, y: 9}));
        party.push(self.offset_pos(Coords {x: 0, y: 6}));
        party.push(self.offset_pos(Coords {x: 3, y: 12}));
        party.push(self.offset_pos(Coords {x: 4, y: 6}));
        party.push(self.offset_pos(Coords {x: 3, y: 5}));
        party.push(self.offset_pos(Coords {x: 3, y: 2}));
        party.push(self.offset_pos(Coords {x: 34, y: 6}));
        party.push(self.offset_pos(Coords {x: 3, y: 43}));
        party.push(self.offset_pos(Coords {x: 3, y: 14}));
        party.push(self.offset_pos(Coords {x: 32, y: 56}));
        party.push(self.offset_pos(Coords {x: 23, y: 16}));
        party.push(self.offset_pos(Coords {x: 32, y: 6}));
        party.push(self.offset_pos(Coords {x: 13, y: 16}));
        party.push(self.offset_pos(Coords {x: 23, y: 43}));
        party.push(self.offset_pos(Coords {x: 23, y: 16}));
        party.push(self.offset_pos(Coords {x: 13, y: 56}));
        party.push(self.offset_pos(Coords {x: 63, y: 46}));
        party.push(self.offset_pos(Coords {x: 73, y: 46}));
        party.push(self.offset_pos(Coords {x: 13, y: 16}));
        party.push(self.offset_pos(Coords {x: 13, y: 17}));
        party.push(self.offset_pos(Coords {x: 13, y: 18}));
        party.push(self.offset_pos(Coords {x: 13, y: 19}));
        party.push(self.offset_pos(Coords {x: 31, y: 6}));
        party.push(self.offset_pos(Coords {x: 31, y: 7}));
        party.push(self.offset_pos(Coords {x: 31, y: 8}));
        party.push(self.offset_pos(Coords {x: 37, y: 14}));
        party.push(self.offset_pos(Coords {x: 37, y: 15}));
        party.push(self.offset_pos(Coords {x: 37, y: 16}));
        party.push(self.offset_pos(Coords {x: 32, y: 26}));
        party.push(self.offset_pos(Coords {x: 31, y: 36}));
        party.push(self.offset_pos(Coords {x: 31, y: 46}));
        party.push(self.offset_pos(Coords {x: 32, y: 56}));
        party.push(self.offset_pos(Coords {x: 31, y: 7}));
        party.push(self.offset_pos(Coords {x: 63, y: 8}));
        party.push(self.offset_pos(Coords {x: 45, y: 9}));
        party.push(self.offset_pos(Coords {x: 12, y: 0}));
        


        party
    }

    pub fn glider(&self) -> Vec<i32> {
        let mut glider : Vec<i32> = Vec::new();

        glider.push(self.offset_pos(Coords {x: 0, y: 0}));
        glider.push(self.offset_pos(Coords {x: 0, y: 2}));
        glider.push(self.offset_pos(Coords {x: 1, y: 1}));
        glider.push(self.offset_pos(Coords {x: 1, y: 2}));
        glider.push(self.offset_pos(Coords {x: 2, y: 1}));

        glider
    }

    pub fn gosper_glider_gun(&self) -> Vec<i32> {
        let mut gosper_glider_gun : Vec<i32> = Vec::new();

        gosper_glider_gun.push(self.offset_pos(Coords {x: 5, y: 1}));
        gosper_glider_gun.push(self.offset_pos(Coords {x: 6, y: 1}));
        gosper_glider_gun.push(self.offset_pos(Coords {x: 5, y: 2}));
        gosper_glider_gun.push(self.offset_pos(Coords {x: 6, y: 2}));
        gosper_glider_gun.push(self.offset_pos(Coords {x: 5, y: 11}));
        gosper_glider_gun.push(self.offset_pos(Coords {x: 6, y: 11}));
        gosper_glider_gun.push(self.offset_pos(Coords {x: 7, y: 11}));
        gosper_glider_gun.push(self.offset_pos(Coords {x: 8, y: 12}));
        gosper_glider_gun.push(self.offset_pos(Coords {x: 4, y: 12}));
        gosper_glider_gun.push(self.offset_pos(Coords {x: 9, y: 13}));
        gosper_glider_gun.push(self.offset_pos(Coords {x: 3, y: 13}));        
        gosper_glider_gun.push(self.offset_pos(Coords {x: 9, y: 14}));
        gosper_glider_gun.push(self.offset_pos(Coords {x: 3, y: 14}));
        gosper_glider_gun.push(self.offset_pos(Coords {x: 6, y: 15}));
        gosper_glider_gun.push(self.offset_pos(Coords {x: 4, y: 16}));
        gosper_glider_gun.push(self.offset_pos(Coords {x: 8, y: 16}));
        gosper_glider_gun.push(self.offset_pos(Coords {x: 5, y: 17}));
        gosper_glider_gun.push(self.offset_pos(Coords {x: 6, y: 17}));
        gosper_glider_gun.push(self.offset_pos(Coords {x: 7, y: 17}));
        gosper_glider_gun.push(self.offset_pos(Coords {x: 6, y: 18}));
        gosper_glider_gun.push(self.offset_pos(Coords {x: 3, y: 21}));
        gosper_glider_gun.push(self.offset_pos(Coords {x: 4, y: 21}));
        gosper_glider_gun.push(self.offset_pos(Coords {x: 5, y: 21}));
        gosper_glider_gun.push(self.offset_pos(Coords {x: 3, y: 22}));
        gosper_glider_gun.push(self.offset_pos(Coords {x: 4, y: 22}));
        gosper_glider_gun.push(self.offset_pos(Coords {x: 5, y: 22}));
        gosper_glider_gun.push(self.offset_pos(Coords {x: 2, y: 23}));
        gosper_glider_gun.push(self.offset_pos(Coords {x: 6, y: 23}));
        gosper_glider_gun.push(self.offset_pos(Coords {x: 1, y: 25}));
        gosper_glider_gun.push(self.offset_pos(Coords {x: 2, y: 25}));
        gosper_glider_gun.push(self.offset_pos(Coords {x: 6, y: 25}));
        gosper_glider_gun.push(self.offset_pos(Coords {x: 7, y: 25}));
        gosper_glider_gun.push(self.offset_pos(Coords {x: 3, y: 35}));
        gosper_glider_gun.push(self.offset_pos(Coords {x: 4, y: 35}));
        gosper_glider_gun.push(self.offset_pos(Coords {x: 3, y: 36}));
        gosper_glider_gun.push(self.offset_pos(Coords {x: 4, y: 36}));
        

        gosper_glider_gun
    }

}

#[wasm_bindgen]
pub fn new_map(x: i32, y: i32) -> Map {
    Map::new(Coords {x, y})
}

#[wasm_bindgen]
pub fn ver1() -> String {
    String::from("0.0.2")
}

