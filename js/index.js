const screen = document.getElementById("main-canvas");
const screenContext = screen.getContext("2d");

export function draw_line(x1, y1, x2, y2, css_color) {
    screenContext.strokeStyle = css_color;
    screenContext.beginPath();
    screenContext.moveTo(x1, y1);
    screenContext.lineTo(x2, y2);
    screenContext.stroke();
}

export function clear_screen() {
  screenContext.clearRect(0, 0, 640, 480);
}

(async function main() {
  try {
    const { GameData, Game } = await import("/pkg/index.js");
    const data = GameData.new();
    const game = Game.new();

    document.addEventListener("keyup", e => {
      switch (e.key) {
        case "ArrowUp":
          game.move_player(0, 1);
          break;
        case "ArrowDown":
          game.move_player(0, -1);
          break;
        case "ArrowLeft":
          game.move_player(1, 0);
          break;
        case "ArrowRight":
          game.move_player(-1, 0);
          break;
      }
    })

    screen.width = data.screen_width();
    screen.height = data.screen_height();
    screen.style.border = "1px solid black";

    const gameLoop = () => {
      clear_screen();
      game.ray_casting();

      requestAnimationFrame(gameLoop);
    }

    gameLoop();

  } catch (err) {
    console.error(err);
  }
})();

