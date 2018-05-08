import * as PIXI from 'pixi.js';
const wasm = import('../rust-module/pkg/conways_game_of_life');
function randomColor() {
  return parseInt((16777216 * Math.random()).toString(16), 16);
}

const xMap = 40;
const yMap = 30;
const xTrue = xMap * 4;
const yTrue = yMap * 4;
const cellSide = 20;

const deadFill = 0xF2F2F2;
const aliveFill = randomColor();

class Cell {
  constructor(x, y, width, height) {
    this.isAlive = false;
    this.x = x;
    this.y = y;
    this.width = width;
    this.height = height;

    this.changeState = (graphics) => {
      if (this.isAlive) {
        this.isAlive = false;
        graphics.beginFill(deadFill);
        graphics.drawRect(x + 0.5, y + 0.5, width - 1, height - 1);
        graphics.endFill();
      } else {
        this.isAlive = true;
        graphics.beginFill(aliveFill);
        graphics.drawRect(x + 0.5, y + 0.5, width - 1, height - 1);
        graphics.endFill();
      }
    };

    this.setAlive = (graphics) => {
      this.isAlive = true;
      graphics.beginFill(aliveFill);
      graphics.drawRect(x + 0.5, y + 0.5, width - 1, height - 1);
      graphics.endFill();
    };

    this.setDead = () => {
      this.isAlive = false;
    };
  }
}

wasm.then((conways) => {
  const app = new PIXI.Application({
    width: 800.5,
    height: 600.5,
  });
  const map = conways.new_map(xMap, yMap);
  document.body.appendChild(app.view);

  const container = new PIXI.Container();
  container.interactive = true;

  const grid = new PIXI.Graphics();
  grid.lineStyle(1, 0x141414);

  const alivesGrid = new PIXI.Graphics();
  alivesGrid.lineStyle(1, 0x141414);

  container.addChild(grid);
  container.addChild(alivesGrid);
  app.stage.addChild(container);

  grid.beginFill(deadFill);

  const cellArr = [];

  for (let x = 0; x < xMap; x += 1) {
    cellArr[x] = [];
    for (let y = 0; y < yMap; y += 1) {
      const cell = new Cell(x * cellSide, y * cellSide, cellSide, cellSide);
      cellArr[x][y] = cell;

      grid.drawRect(cell.x, cell.y, cell.width, cell.height);
    }
  }

  grid.endFill();

  container.on('click', (e) => {
    const clickX = e.data.global.x;
    const clickY = e.data.global.y;

    const cellX = parseInt(clickX / cellSide, 10);
    const cellY = parseInt(clickY / cellSide, 10);

    const cell = cellArr[cellX][cellY];

    cell.changeState(alivesGrid);
  });

  const randomMap = () => {
    const random = [];

    for (let i = 0; i < 10000; i += 1) {
      const x = Math.floor(Math.random() * xTrue);
      const y = Math.floor(Math.random() * yTrue);

      const pos = y + (x * yTrue);
      random.push(pos);
    }

    return random;
  };

  map.set_alive(randomMap());

  const defineMap = (prevAlives, alives) => {
    alivesGrid.clear();

    for (let i = 0; i < prevAlives.length; i += 1) {
      const x = parseInt(prevAlives[i] / yMap, 10);
      const y = parseInt(prevAlives[i] % yMap, 10);
      if (x < xMap && y < yMap) {
        cellArr[x][y].setDead();
      }
    }

    for (let i = 0; i < alives.length; i += 1) {
      const x = parseInt(alives[i] / yMap, 10);
      const y = parseInt(alives[i] % yMap, 10);

      if (x < xMap && y < yMap) {
        cellArr[x][y].setAlive(alivesGrid);
      }
    }

    return alives;
  };

  let prev = defineMap([], map.get_map_alives());

  setInterval(() => {
    map.next_tick();
    prev = defineMap(prev, map.get_map_alives());
  }, 300);
});
