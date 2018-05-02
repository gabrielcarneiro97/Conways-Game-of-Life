/* tslint:disable */
export enum State {Alive,Dead,}
export function new_map(arg0: number, arg1: number): Map;

export class Cell {
free(): void;
static  new(arg0: number, arg1: number, arg2: Int32Array): Cell;

 change_state(): void;

}
export class Coords {
free(): void;
}
export class Map {
free(): void;
static  new(arg0: Coords): Map;

 get_alives(): Int32Array;

 next_tick(): void;

 find_neighboors(arg0: number): Int32Array;

 count_neighboors_alive(arg0: Cell): number;

 get_map(): Int32Array;

 get_map_alives(): Int32Array;

 get_pos(arg0: Coords): number;

 get_coords(arg0: number): Coords;

 is_alive(arg0: number): boolean;

 set_alive(arg0: Int32Array): void;

 offset_pos(arg0: Coords): number;

 blinker(): Int32Array;

 glider(): Int32Array;

 gosper_glider_gun(): Int32Array;

}
