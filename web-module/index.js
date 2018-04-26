import * as PIXI from 'pixi.js'

const wasm = import('conways_game_of_life/conways_game_of_life')
const x_map = 80
const y_map = 60
const cellSide = 10

const dead_fill = 0xF2F2F2
const alive_fill = 0x3C8C30
const was_alive_fill = 0xC5FFD7

class Cell {
  constructor(x, y, width, height) {

    this.isAlive = false

    this.x = x
    this.y = y
    this.width = width
    this.height = height

    this.change_state = (graphics) => { 
      if (this.isAlive) {
        this.isAlive = false
        graphics.beginFill(dead_fill)
        graphics.drawRect(x, y, width, height)
        graphics.endFill()
      } else {
        this.isAlive = true
        graphics.beginFill(alive_fill)
        graphics.drawRect(x, y, width, height)
        graphics.endFill()
      }
    }

    this.set_alive = (graphics) => {
      this.isAlive = true
      graphics.beginFill(alive_fill)
      graphics.drawRect(x, y, width, height)
      graphics.endFill()
    }

    this.set_dead = (graphics) => {
      this.isAlive = false
      graphics.beginFill(was_alive_fill)
      graphics.drawRect(x, y, width, height)
      graphics.endFill()
    } 
  }

}

wasm.then(conways => {
  const app = new PIXI.Application()
  let map = conways.new_map(x_map, y_map)  
  document.body.appendChild(app.view)
  
  let container = new PIXI.Container()
  container.interactive = true

  let graphics = new PIXI.Graphics()
  graphics.lineStyle(1, 0x141414)  

  container.addChild(graphics)
  app.stage.addChild(container)

  graphics.beginFill(dead_fill)

  let cell_arr = []

  for (let x = 0; x < x_map; x++) {
    cell_arr[x] = []
    for (let y = 0; y < y_map; y++) {
      let cell = new Cell(x * cellSide, y * cellSide, cellSide, cellSide)
      cell_arr[x][y] = cell

      graphics.drawRect(cell.x, cell.y, cell.width, cell.height)
    }
  }

  graphics.endFill()


  container.on('click', (e) => {
    let click_x = e.data.global.x
    let click_y = e.data.global.y

    let cell_x = parseInt(click_x / cellSide)
    let cell_y = parseInt(click_y / cellSide)

    let cell = cell_arr[cell_x][cell_y]

    cell.change_state(graphics)


  })

  map.set_alive(map.gosper_glider_gun())



  let define_map = (prev_alives, alives) => {

    for (let i = 0; i < prev_alives.length; i ++) {
      let x = parseInt(prev_alives[i] % y_map)
      let y = parseInt(prev_alives[i] / y_map)

      cell_arr[x][y].set_dead(graphics)
    }

    for (let i = 0; i < alives.length; i++) {
      let x = parseInt(alives[i] % y_map)
      let y = parseInt(alives[i] / y_map)

    cell_arr[x][y].set_alive(graphics)

    }

    return alives
  } 

  let prev = define_map([], map.get_map_alives())

  setInterval(() => {
  map.next_tick()
  prev = define_map(prev, map.get_map_alives())

  }, 100)


})

