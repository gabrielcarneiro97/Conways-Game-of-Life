// const worker = new Worker('./worker.js')

// worker.postMessage('wow')

// worker.onmessage = (m) => {
//   console.log(m)
// }

let sideCellNum = document.getElementById('side').value
const canvasSide = window.innerHeight - 50
let cellSide = canvasSide / sideCellNum

var renderer = PIXI.autoDetectRenderer(canvasSide, canvasSide, { backgroundColor: 0x1099bb })
document.getElementById('canvas').appendChild(renderer.view)
// create the root of the scene graph
var stage = new PIXI.Container()
var count = 0
// create a new Graphics object
var graphics = new PIXI.Graphics()
// center the graphic
graphics.x = 200
graphics.y = 150
// set a fill color and an opacity
graphics.beginFill(0xfff012, 1)
// draw a rectangle using the arguments as: x, y, width, height
graphics.drawRect(0, 0, 50, 50)
// center the pivot point
graphics.pivot.x = 25
graphics.pivot.y = 25
// add it to your scene
stage.addChild(graphics)
// start animating
renderer.render(stage)

// animate()
// function animate () {
//   requestAnimationFrame(animate)
//   count += 0.08;
//   // more fun when it moves
//   graphics.rotation = Math.sin(count) * 0.2
//   // render the container
//   renderer.render(stage)
// }

document.getElementById('btn-set').onclick = () => {
  sideCellNum = document.getElementById('side').value

  console.log(sideCellNum)
}
