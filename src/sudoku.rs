use rsat::msat::*;
use rsat::{Lit, Solution};

pub struct Sudoku {
    pub grid: [[u32; 9]; 9],
    solver: Solver,
}

impl Sudoku {
    pub fn new(grid: [[u32; 9]; 9]) -> Option<Self> {
        let mut solver = Solver::new(SolverOptions::default());
        let mut lits = [[[Lit::new(0, false); 9]; 9]; 9];

        for lits_i in &mut lits {
            for lits_ij in lits_i.iter_mut() {
                for lits_ijk in lits_ij.iter_mut() {
                    // Cell (i, j) is assigned k+1
                    *lits_ijk = Lit::new(solver.new_var(), false);
                }
            }
        }

        // Exactly one value is assigned to each cell
        // Each horizontal line contains k exactly once
        // Each vertical line contains k exactly once
        // Each 3x3 grid contains k exactly once
        for i in 0..9 {
            for j in 0..9 {
                let mut cl = vec![];
                for k in 0..9 {
                    cl.push(lits[i][j][k]);
                    for l in 0..9 {
                        if k != l {
                            // Cell(i, j) == k+1 => Cell(i, j) != l+1 for k != l
                            if !solver.new_clause(vec![!lits[i][j][k], !lits[i][j][l]]) {
                                return None;
                            }
                        }
                        if j != l {
                            // Cell(i, j) == k+1 => Cell(i, l) != k+1 for j != l
                            if !solver.new_clause(vec![!lits[i][j][k], !lits[i][l][k]]) {
                                return None;
                            }
                        }
                        if i != l {
                            // Cell(i, j) == k+1 => Cell(l, j) != k+1 for i != l
                            if !solver.new_clause(vec![!lits[i][j][k], !lits[l][j][k]]) {
                                return None;
                            }
                        }

                        let mod_i = (i / 3) * 3 + l / 3;
                        let mod_j = (j / 3) * 3 + l % 3;
                        if i != mod_i || j != mod_j {
                            // Cell(i, j) == k+1 => Cell(mod_i, mod_j) != k+1 for i != mod_i, j != mod_j
                            if !solver.new_clause(vec![!lits[i][j][k], !lits[mod_i][mod_j][k]]) {
                                return None;
                            }
                        }
                    }
                }

                // At least one of 1..=9 is assigned to Cell(i, j)
                if !solver.new_clause(cl) {
                    return None;
                }

                if grid[i][j] != 0 {
                    // Unit clause for already assigned cells
                    if !solver.new_clause(vec![lits[i][j][grid[i][j] as usize - 1]]) {
                        return None;
                    }
                }
            }
        }

        Some(Sudoku { grid, solver })
    }

    pub fn solve(&mut self) -> bool {
        match self.solver.solve(vec![]) {
            Solution::Sat(sol) => {
                for i in 0..9 {
                    for j in 0..9 {
                        for k in 0..9 {
                            if sol[9 * 9 * i + 9 * j + k] {
                                if self.grid[i][j] != 0 && self.grid[i][j] != k as u32 + 1 {
                                    panic!("This should not occur. Something wrong in solver.");
                                }
                                self.grid[i][j] = k as u32 + 1;
                            }
                        }
                    }
                }
                true
            }
            Solution::Unsat | Solution::Unknown | Solution::Best(_) => false,
        }
    }
}
