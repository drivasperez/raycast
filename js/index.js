const screen = document.getElementById("main-canvas");
const screenContext = screen.getContext("2d");
screenContext.imageSmoothingEnabled = false;
const heldKeys = new Set();

let paused = false;


export function draw_line(x1, y1, x2, y2, css_color) {
  screenContext.strokeStyle = css_color;
  screenContext.beginPath();
  screenContext.moveTo(x1, y1);
  screenContext.lineTo(x2, y2);
  screenContext.stroke();
}

export function clear_screen(width, height) {
  screenContext.clearRect(0, 0, width, height);
}

export function set_stroke_style(style) {
  screenContext.strokeStyle = style;
}

export function load_texture_data(id, width, height) {
  let image = document.getElementById(id);
  let canvas = document.createElement('canvas');
  canvas.width = texture.width;
  canvas.height = texture.height;

  let canvasContext = canvas.getContext("2d");
  canvasContext.drawImage(image, 0, 0, width, height);
  const imageData = canvasContext.getImageData(0, 0, width, height).data;
  return imageData;
}

function addHeldInputs(event) {
  heldKeys.add(event.which);
}

function removeHeldInputs(event) {
  heldKeys.delete(event.which);
}

async function main() {
  try {
    const { Game } = await import("/pkg/index.js");
    const { memory } = await import("/pkg/index_bg.wasm");

    const game = Game.new();
    const data = game.data();

    const inputsPointer = game.inputs_ptr();
    const inputs = new Uint32Array(memory.buffer, inputsPointer, 16);

    const screenBufferLength = game.screen_buffer_len();
    const screenBufferPtr = game.screen_buffer_ptr();

    screen.width = data.screen_width();
    screen.height = data.screen_height();
    screen.style.border = "1px solid black";

    const screenBuffer = new Uint8ClampedArray(memory.buffer, screenBufferPtr, screenBufferLength);
    console.log("screenBuffer length", screenBuffer.length);
    const screenImageData = new ImageData(screenBuffer, data.projection_width(), data.projection_height());

    const renderBuffer = () => {
      let temp = document.createElement('canvas');
      temp.width = screen.width;
      temp.height = screen.height;
      temp.getContext('2d').putImageData(screenImageData, 0, 0);
      screenContext.drawImage(temp, 0, 0);
    }

    const setInputs = () => {
      const held = [...heldKeys];
      for (let i = 0; i < 16; i++) {
        inputs[i] = held[i] || 0;
      }
    }


    const screenScale = data.scale();
    screenContext.scale(screenScale, screenScale);
    screenContext.translate(0.5, 0.5);

    document.addEventListener("keydown", addHeldInputs);
    document.addEventListener("keyup", removeHeldInputs);

    const projectionWidth = data.projection_width();
    const projectionHeight = data.projection_width();

    const gameLoop = () => {
      if (!paused) {
        clear_screen(projectionWidth, projectionHeight);
        setInputs();
        game.tick();
        renderBuffer();
      }

      requestAnimationFrame(gameLoop);
    }

    gameLoop();

  } catch (err) {
    console.error(err);
  }
};

window.onload = function () {
  main();
}


window.addEventListener('blur', event => {
  if (!paused) {
    paused = true;
    renderFocusLost();
  }
})

window.addEventListener('focus', event => {
  paused = false;
})

function renderFocusLost() {
  screenContext.fillStyle = 'rgba(0,0,0,0.5)';
  screenContext.fillRect(0, 0, screen.width, screen.height);
  screenContext.fillStyle = 'white';
  screenContext.font = '10px sans-serif';
  screenContext.fillText('CLICK TO FOCUS', 37, screen.height / 2);
}