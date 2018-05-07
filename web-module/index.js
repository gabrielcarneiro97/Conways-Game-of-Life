import * as PIXI from 'pixi.js'

const wasm = import('conways_game_of_life/conways_game_of_life')
const xMap = 40
const yMap = 30
const xTrue = xMap * 4
const yTrue = yMap * 4
const cellSide = 20

const deadFill = 0xF2F2F2
const aliveFill = parseInt(((1 << 24) * Math.random() | 0).toString(16), 16)

class Cell {
  constructor (x, y, width, height) {
    this.isAlive = false

    this.x = x
    this.y = y
    this.width = width
    this.height = height

    this.change_state = (graphics) => {
      if (this.isAlive) {
        this.isAlive = false
        graphics.beginFill(deadFill)
        graphics.drawRect(x + 0.5, y + 0.5, width - 1, height - 1)
        graphics.endFill()
      } else {
        this.isAlive = true
        graphics.beginFill(aliveFill)
        graphics.drawRect(x + 0.5, y + 0.5, width - 1, height - 1)
        graphics.endFill()
      }
    }

    this.set_alive = (graphics) => {
      this.isAlive = true
      graphics.beginFill(aliveFill)
      graphics.drawRect(x + 0.5, y + 0.5, width - 1, height - 1)
      graphics.endFill()
    }

    this.set_dead = () => {
      this.isAlive = false
    }
  }
}

wasm.then(conways => {
  const app = new PIXI.Application()
  let map = conways.new_map(xMap, yMap)
  document.body.appendChild(app.view)

  let container = new PIXI.Container()
  container.interactive = true

  let grid = new PIXI.Graphics()
  grid.lineStyle(1, 0x141414)

  let alivesGrid = new PIXI.Graphics()
  alivesGrid.lineStyle(1, 0x141414)

  container.addChild(grid)
  container.addChild(alivesGrid)
  app.stage.addChild(container)

  grid.beginFill(deadFill)

  let cellArr = []

  for (let x = 0; x < xMap; x++) {
    cellArr[x] = []
    for (let y = 0; y < yMap; y++) {
      let cell = new Cell(x * cellSide, y * cellSide, cellSide, cellSide)
      cellArr[x][y] = cell

      grid.drawRect(cell.x, cell.y, cell.width, cell.height)
    }
  }

  grid.endFill()

  container.on('click', (e) => {
    let clickX = e.data.global.x
    let clickY = e.data.global.y

    let cellX = parseInt(clickX / cellSide)
    let cellY = parseInt(clickY / cellSide)

    let cell = cellArr[cellX][cellY]

    cell.change_state(alivesGrid)
  })

  let randomMap = () => {
    let random = []

    for (let i = 0; i < 10000; i++) {
      let x = Math.floor(Math.random() * xTrue)
      let y = Math.floor(Math.random() * yTrue)

      let pos = y + (x * yTrue)
      random.push(pos)
    }

    return random
  }

  map.set_alive(randomMap())

  let defineMap = (prevAlives, alives) => {
    alivesGrid.clear()

    for (let i = 0; i < prevAlives.length; i++) {
      let x = parseInt(prevAlives[i] / yMap)
      let y = parseInt(prevAlives[i] % yMap)
      if (x < xMap && y < yMap) {
        cellArr[x][y].set_dead()
      }
    }

    for (let i = 0; i < alives.length; i++) {
      let x = parseInt(alives[i] / yMap)
      let y = parseInt(alives[i] % yMap)

      if (x < xMap && y < yMap) {
        cellArr[x][y].set_alive(alivesGrid)
      }
    }

    return alives
  }

  let prev = defineMap([], map.get_map_alives())

  setInterval(() => {
    map.next_tick()
    prev = defineMap(prev, map.get_map_alives())
  }, 300)
})
