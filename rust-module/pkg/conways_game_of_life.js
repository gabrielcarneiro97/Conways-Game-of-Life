/* tslint:disable */
import * as wasm from './conways_game_of_life_bg';

export const State = Object.freeze({ Alive:0,Dead:1, });

let cachedUint32Memory = null;
function getUint32Memory() {
    if (cachedUint32Memory === null ||
        cachedUint32Memory.buffer !== wasm.memory.buffer)
        cachedUint32Memory = new Uint32Array(wasm.memory.buffer);
    return cachedUint32Memory;
}

function passArray32ToWasm(arg) {
    const ptr = wasm.__wbindgen_malloc(arg.length * 4);
    getUint32Memory().set(arg, ptr / 4);
    return [ptr, arg.length];
}

let cachedGlobalArgumentPtr = null;
function globalArgumentPtr() {
    if (cachedGlobalArgumentPtr === null)
        cachedGlobalArgumentPtr = wasm.__wbindgen_global_argument_ptr();
    return cachedGlobalArgumentPtr;
}

function setGlobalArgument(arg, i) {
    const idx = globalArgumentPtr() / 4 + i;
    getUint32Memory()[idx] = arg;
}

function getArrayI32FromWasm(ptr, len) {
    const mem = getUint32Memory();
    const slice = mem.slice(ptr / 4, ptr / 4 + len);
    return new Int32Array(slice);
}

function getGlobalArgument(arg) {
    const idx = globalArgumentPtr() / 4 + arg;
    return getUint32Memory()[idx];
}

export function new_map(arg0, arg1) {
    return Map.__construct(wasm.new_map(arg0, arg1));
}

export class Cell {

                static __construct(ptr) {
                    return new Cell(ptr);
                }

                constructor(ptr) {
                    this.ptr = ptr;
                }

            free() {
                const ptr = this.ptr;
                this.ptr = 0;
                wasm.__wbg_cell_free(ptr);
            }
        static new(arg0, arg1, arg2) {
    const [ptr2, len2] = passArray32ToWasm(arg2);
    setGlobalArgument(len2, 0);
    return Cell.__construct(wasm.cell_new(arg0, arg1, ptr2));
}
change_state() {
    return wasm.cell_change_state(this.ptr);
}
}

export class Coords {

                static __construct(ptr) {
                    return new Coords(ptr);
                }

                constructor(ptr) {
                    this.ptr = ptr;
                }

            free() {
                const ptr = this.ptr;
                this.ptr = 0;
                wasm.__wbg_coords_free(ptr);
            }
        }

export class Map {

                static __construct(ptr) {
                    return new Map(ptr);
                }

                constructor(ptr) {
                    this.ptr = ptr;
                }

            free() {
                const ptr = this.ptr;
                this.ptr = 0;
                wasm.__wbg_map_free(ptr);
            }
        static new(arg0) {
    const ptr0 = arg0.ptr;
    arg0.ptr = 0;
    return Map.__construct(wasm.map_new(ptr0));
}
get_alives() {
    const ret = wasm.map_get_alives(this.ptr);
    const len = getGlobalArgument(0);
    const realRet = getArrayI32FromWasm(ret, len);
    wasm.__wbindgen_free(ret, len * 4);
    return realRet;
}
next_tick() {
    return wasm.map_next_tick(this.ptr);
}
find_neighboors(arg0) {
    const ret = wasm.map_find_neighboors(this.ptr, arg0);
    const len = getGlobalArgument(0);
    const realRet = getArrayI32FromWasm(ret, len);
    wasm.__wbindgen_free(ret, len * 4);
    return realRet;
}
count_neighboors_alive(arg0) {
    return wasm.map_count_neighboors_alive(this.ptr, arg0.ptr);
}
get_map() {
    const ret = wasm.map_get_map(this.ptr);
    const len = getGlobalArgument(0);
    const realRet = getArrayI32FromWasm(ret, len);
    wasm.__wbindgen_free(ret, len * 4);
    return realRet;
}
get_map_alives() {
    const ret = wasm.map_get_map_alives(this.ptr);
    const len = getGlobalArgument(0);
    const realRet = getArrayI32FromWasm(ret, len);
    wasm.__wbindgen_free(ret, len * 4);
    return realRet;
}
get_pos(arg0) {
    return wasm.map_get_pos(this.ptr, arg0.ptr);
}
get_coords(arg0) {
    return Coords.__construct(wasm.map_get_coords(this.ptr, arg0));
}
is_alive(arg0) {
    return (wasm.map_is_alive(this.ptr, arg0)) !== 0;
}
set_alive(arg0) {
    const [ptr0, len0] = passArray32ToWasm(arg0);
    setGlobalArgument(len0, 0);
    return wasm.map_set_alive(this.ptr, ptr0);
}
offset_pos(arg0) {
    const ptr0 = arg0.ptr;
    arg0.ptr = 0;
    return wasm.map_offset_pos(this.ptr, ptr0);
}
blinker() {
    const ret = wasm.map_blinker(this.ptr);
    const len = getGlobalArgument(0);
    const realRet = getArrayI32FromWasm(ret, len);
    wasm.__wbindgen_free(ret, len * 4);
    return realRet;
}
glider() {
    const ret = wasm.map_glider(this.ptr);
    const len = getGlobalArgument(0);
    const realRet = getArrayI32FromWasm(ret, len);
    wasm.__wbindgen_free(ret, len * 4);
    return realRet;
}
gosper_glider_gun() {
    const ret = wasm.map_gosper_glider_gun(this.ptr);
    const len = getGlobalArgument(0);
    const realRet = getArrayI32FromWasm(ret, len);
    wasm.__wbindgen_free(ret, len * 4);
    return realRet;
}
}

let slab = [];

let slab_next = 0;

function addHeapObject(obj) {
    if (slab_next === slab.length)
        slab.push(slab.length + 1);
    const idx = slab_next;
    const next = slab[idx];

    slab_next = next;

    slab[idx] = { obj, cnt: 1 };
    return idx << 1;
}

let stack = [];

function getObject(idx) {
    if ((idx & 1) === 1) {
        return stack[idx >> 1];
    } else {
        const val = slab[idx >> 1];

    return val.obj;

    }
}

export function __wbindgen_object_clone_ref(idx) {
    // If this object is on the stack promote it to the heap.
    if ((idx & 1) === 1)
        return addHeapObject(getObject(idx));

    // Otherwise if the object is on the heap just bump the
    // refcount and move on
    const val = slab[idx >> 1];
    val.cnt += 1;
    return idx;
}

function dropRef(idx) {

    let obj = slab[idx >> 1];

    obj.cnt -= 1;
    if (obj.cnt > 0)
        return;

    // If we hit 0 then free up our space in the slab
    slab[idx >> 1] = slab_next;
    slab_next = idx >> 1;
}

export function __wbindgen_object_drop_ref(i) { dropRef(i); }

const TextDecoder = typeof self === 'object' && self.TextDecoder
    ? self.TextDecoder
    : require('util').TextDecoder;

let cachedDecoder = new TextDecoder('utf-8');

let cachedUint8Memory = null;
function getUint8Memory() {
    if (cachedUint8Memory === null ||
        cachedUint8Memory.buffer !== wasm.memory.buffer)
        cachedUint8Memory = new Uint8Array(wasm.memory.buffer);
    return cachedUint8Memory;
}

function getStringFromWasm(ptr, len) {
    return cachedDecoder.decode(getUint8Memory().slice(ptr, ptr + len));
}

export function __wbindgen_string_new(p, l) {
    return addHeapObject(getStringFromWasm(p, l));
}

export function __wbindgen_number_new(i) { return addHeapObject(i); }

export function __wbindgen_number_get(n, invalid) {
    let obj = getObject(n);
    if (typeof(obj) === 'number')
        return obj;
    getUint8Memory()[invalid] = 1;
    return 0;
}

export function __wbindgen_undefined_new() { return addHeapObject(undefined); }

export function __wbindgen_null_new() {
    return addHeapObject(null);
}

export function __wbindgen_is_null(idx) {
    return getObject(idx) === null ? 1 : 0;
}

export function __wbindgen_is_undefined(idx) {
    return getObject(idx) === undefined ? 1 : 0;
}

export function __wbindgen_boolean_new(v) {
    return addHeapObject(v === 1);
}

export function __wbindgen_boolean_get(i) {
    let v = getObject(i);
    if (typeof(v) === 'boolean') {
        return v ? 1 : 0;
    } else {
        return 2;
    }
}

export function __wbindgen_symbol_new(ptr, len) {
    let a;
    if (ptr === 0) {
        a = Symbol();
    } else {
        a = Symbol(getStringFromWasm(ptr, len));
    }
    return addHeapObject(a);
}

export function __wbindgen_is_symbol(i) {
    return typeof(getObject(i)) === 'symbol' ? 1 : 0;
}

const TextEncoder = typeof self === 'object' && self.TextEncoder
    ? self.TextEncoder
    : require('util').TextEncoder;

let cachedEncoder = new TextEncoder('utf-8');

function passStringToWasm(arg) {

    const buf = cachedEncoder.encode(arg);
    const ptr = wasm.__wbindgen_malloc(buf.length);
    getUint8Memory().set(buf, ptr);
    return [ptr, buf.length];
}

export function __wbindgen_string_get(i, len_ptr) {
    let obj = getObject(i);
    if (typeof(obj) !== 'string')
        return 0;
    const [ptr, len] = passStringToWasm(obj);
    getUint32Memory()[len_ptr / 4] = len;
    return ptr;
}

export function __wbindgen_throw(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
}

