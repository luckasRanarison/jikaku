import init, { matchKanji } from "./pkg/jikaku_wasm.js";

await init();

/** @type HTMLCanvasElement **/
const canvasElem = document.getElementById("canvas");
const matchButton = document.getElementById("btn-match");
const clearButton = document.getElementById("btn-clear");
const matchContainer = document.getElementById("match-container");

/** @type CanvasRenderingContext2D **/
const ctx = canvasElem.getContext("2d");
const canvasWidth = canvasElem.width;
const canvasHeight = canvasElem.height;

let isDrawing = false;
let strokes = [];

ctx.lineWidth = 4;
ctx.lineCap = "round";
ctx.lineJoin = "round";

canvasElem.addEventListener("mousedown", (ev) => {
  isDrawing = true;
  strokes.push([[ev.offsetX, ev.offsetY]]);
});

canvasElem.addEventListener("mouseup", (_ev) => {
  isDrawing = false;
});

canvasElem.addEventListener("mousemove", (ev) => {
  if (!isDrawing) return;
  const lastStroke = strokes[strokes.length - 1];
  lastStroke.push([ev.offsetX, ev.offsetY]);
  render();
});

clearButton.addEventListener("click", () => {
  strokes = [];
  matchContainer.innerHTML = "";
  render();
});

matchButton.addEventListener("click", () => {
  const candidates = matchKanji(strokes);

  matchContainer.innerHTML = candidates
    .map(
      (c) =>
        `<span class="match" title="distance: ${c.distance.toFixed(2)}">
          ${String.fromCharCode(c.id)}
        </span>`,
    )
    .join("");
});

function render() {
  ctx.clearRect(0, 0, canvasWidth, canvasHeight);

  for (const stroke of strokes) {
    drawStroke(stroke);
  }
}

function drawStroke(points) {
  const path = new Path2D();

  for (let i = 0; i < points.length; i++) {
    const [x, y] = points[i];

    if (i == 0) {
      path.moveTo(x, y);
    } else {
      path.lineTo(x, y);
    }
  }

  ctx.stroke(path);
}
