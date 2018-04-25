const js = import("./conways_game_of_life.js")

let worker = new Worker('./worker.js')

js.then(js => {
  let map = js.new_map(500, 500)
  map.set_alive(map.gosper_glider_gun())
  setInterval(() => {
    console.log(map.get_alives())
    map.next_tick()
  }, 0.1)
})

