const rust = import('./pkg/rust_client')
const Stats = require('stats.js')

const canvas = document.getElementById('rustCanvas')
const gl = canvas.getContext('webgl', { antialias: true })

//const FPS_THROTTLE = 1000 / 60
const TICKS_PER_SECOND = 120

updateWindow = _ => {
  if (window.innerWidth !== canvas.width || window.innerHeight !== canvas.height) {
    canvas.height = window.innerHeight;
    canvas.clientHeight = window.innerHeight;
    canvas.style.height = window.innerHeight;

    canvas.width = window.innerWidth;
    canvas.clientWidth = window.innerWidth;
    canvas.style.width = window.innerWidth;

    gl.viewport(0, 0, window.innerWidth, window.innerHeight);
  }
}

rust.then(r => {
  if (!gl) {
    alert('couldnt initialize WebGL')
    return
  }

  let stats = new Stats()
  document.body.appendChild( stats.dom )

  const dt = 1.0 / TICKS_PER_SECOND

  let currentTime = performance.now()
  let accumulator = 0.0

  const client = new r.RustClient()

  tick = _ => {
    stats.begin()
    const newTime = performance.now()
    const frameTime = (newTime - currentTime) / 1000
    currentTime = newTime

    accumulator += frameTime

    while (accumulator >= dt) {
      client.update(dt, window.innerWidth, window.innerHeight)
      accumulator -= dt
    }
    updateWindow()
    window.requestAnimationFrame(tick)
    client.draw()
    stats.end()
  }

  console.log('Begin rendering...')
  tick()
})