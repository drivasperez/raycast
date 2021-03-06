const screen = document.getElementById("main-canvas");
const screenContext = screen.getContext("2d");
const heldKeys = new Set();
const tempBuf = document.createElement('canvas');

let paused = false;


function clearScreen(width, height) {
  screenContext.clearRect(0, 0, width, height);
}

export function set_debug_message(msg) {
  let debug_window = document.getElementById("debug");
  debug_window.innerText = msg;
}

export function load_texture_data(id, width, height) {
  let image = document.getElementById(id);
  let canvas = document.createElement('canvas');
  canvas.width = image.width;
  canvas.height = image.height;

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

    const screenBuffer = new Uint8ClampedArray(memory.buffer, screenBufferPtr, screenBufferLength);
    console.log("screenBuffer length", screenBuffer.length);
    const screenImageData = new ImageData(screenBuffer, data.projection_width(), data.projection_height());

    const renderBuffer = () => {
      tempBuf.width = screen.width;
      tempBuf.height = screen.height;
      tempBuf.getContext('2d').putImageData(screenImageData, 0, 0);
      screenContext.imageSmoothingEnabled = false;
      screenContext.drawImage(tempBuf, 0, 0);
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
        clearScreen(projectionWidth, projectionHeight);
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