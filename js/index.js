const screen = document.getElementById("main-canvas");
const screenContext = screen.getContext("2d");
const heldKeys = new Set();

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

    const setInputs = () => {
      const held = [...heldKeys];
      for (let i = 0; i < 16; i++) {
        inputs[i] = held[i] || 0;
      }
    }

    screen.width = data.screen_width();
    screen.height = data.screen_height();
    screen.style.border = "1px solid black";

    const screenScale = data.scale();
    screenContext.scale(screenScale, screenScale);
    screenContext.translate(0.5, 0.5);

    document.addEventListener("keydown", addHeldInputs);
    document.addEventListener("keyup", removeHeldInputs);

    const projectionWidth = data.projection_width();
    const projectionHeight = data.projection_width();

    const gameLoop = () => {
      clear_screen(projectionWidth, projectionHeight);
      setInputs();
      game.tick();

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
