import Phaser from 'phaser'
import * as wasm from 'conways_game_of_life'

var config = {
  type: Phaser.AUTO,
  parent: 'phaser-example',
  width: 800,
  height: 600,
  scene: {
    preload: preload,
    create: create
  }
}

var game = new Phaser.Game(config)

function preload () {
  this.load.image('logo', 'assets/logo.png')
}

function create () {
  var logo = this.add.image(400, 150, 'logo')

  console.log(wasm)

  this.tweens.add({
    targets: logo,
    y: 450,
    duration: 2000,
    ease: 'Power2',
    yoyo: true,
    loop: -1
  })

}
