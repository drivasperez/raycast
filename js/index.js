const screen = document.getElementById("main-canvas");
const screenContext = screen.getContext("2d");

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
  console.log("Here's the image", imageData);
  return imageData;
}

async function main() {
  try {
    const { Game } = await import("/pkg/index.js");

    const game = Game.new();
    const data = game.data();

    screen.width = data.screen_width();
    screen.height = data.screen_height();
    screen.style.border = "1px solid black";

    const screenScale = data.scale();
    screenContext.scale(screenScale, screenScale);
    screenContext.translate(0.5, 0.5);

    document.addEventListener("keydown", e => {
      game.set_held_key(e.which);
    })


    const projectionWidth = data.projection_width();
    const projectionHeight = data.projection_width();

    const gameLoop = () => {
      clear_screen(projectionWidth, projectionHeight);
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
