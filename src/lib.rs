use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::console;

mod sudoku;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Your code goes here!
    // console::log_1(&JsValue::from_str("Hello world!"));

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct SudokuGrid([[u32; 9]; 9]);

#[derive(Serialize, Deserialize, Debug)]
struct SudokuSolution {
    solved: bool,
    grid: [[u32; 9]; 9],
}

#[wasm_bindgen]
pub fn solve(grid: &JsValue) -> JsValue {
    let grid: SudokuGrid = grid.into_serde().unwrap();
    let mut sudoku = sudoku::Sudoku::new(grid.0);
    let solved = sudoku.solve();
    JsValue::from_serde(&SudokuSolution {
        solved,
        grid: sudoku.grid,
    })
    .unwrap()
}
