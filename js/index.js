const initialGrid = [
  [6, 0, 0, 0, 0, 4, 0, 0, 1],
  [0, 0, 1, 0, 0, 0, 0, 4, 9],
  [5, 0, 0, 0, 1, 0, 0, 0, 0],
  [1, 5, 7, 0, 0, 0, 0, 9, 6],
  [0, 0, 4, 0, 9, 6, 0, 0, 3],
  [3, 0, 0, 0, 4, 5, 0, 1, 8],
  [0, 0, 0, 0, 7, 0, 0, 0, 0],
  [7, 6, 0, 0, 2, 0, 0, 0, 0],
  [0, 0, 8, 5, 0, 0, 3, 0, 4]
];
const inputs = document.querySelectorAll("input");

const setGrid = grid => {
  for (let i = 0; i < 81; i++) {
    const div = (i / 9) >> 0;
    const rem = i % 9;
    const val = grid[div][rem];
    inputs[i].value = val !== 0 ? val : "";
  }
};

setGrid(initialGrid);

document.querySelector("#solve-btn").onclick = async () => {
  try {
    for (let i = 0; i < 81; i++) {
      const div = (i / 9) >> 0;
      const rem = i % 9;
      const val = inputs[i].value;
      initialGrid[div][rem] = Number(val);
    }
    const lib = await import("../pkg/index.js");
    const { solved, grid } = lib.solve(initialGrid);
    if (solved) {
      setGrid(grid);
      document.querySelector("#status").innerHTML = "Solved!";
    } else {
      document.querySelector("#status").innerHTML = "Could not be solved!";
    }
  } catch (error) {
    document.querySelector("#status").innerHTML = "Could not be solved!";
    console.error(error);
  }
};
