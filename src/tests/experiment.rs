use crate::nqueens::NQueens;
use crate::nqueens::representations::{Tuples, ColumnVec};
use crate::nqueens::Heuristic;
use crate::tests::{test_dfs, test_bfs, test_bestfs};
use std::time::Instant;
use std::fs::File;
use std::io::Write;

pub fn test(nmax_dfs: usize, nmax_bfs: usize, nmax_bestfs: &[usize])
{
    let mut file = File::create("results.csv").unwrap();
    writeln!(file, "N;representation;algorithm;heuristic;solution;open_count;closed_count;execution_time(s)").unwrap();
    test_dfs(nmax_dfs, &mut file);
    // test_bfs(nmax_bfs, &mut file);
    file.flush().unwrap();
}