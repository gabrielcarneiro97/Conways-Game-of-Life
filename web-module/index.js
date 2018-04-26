import Phaser from 'phaser'
const wasm =  import('conways_game_of_life/conways_game_of_life')

var config = {
  type: Phaser.AUTO,
  parent: 'phaser-game',
  scene: {
    preload: preload,
    create: create
  }
}


var game = new Phaser.Game(config)

function preload() {
  this.load.image('logo', 'assets/logo.png')
}

function create() {
  var logo = this.add.image(400, 150, 'logo')

  wasm.then(conways => {
    console.log(conways)
    console.log(window.width, window.height)
    this.tweens.add({
      targets: logo,
      y: 450,
      duration: 2000,
      ease: 'Power2',
      yoyo: true,
      loop: -1
    })
  })



}
