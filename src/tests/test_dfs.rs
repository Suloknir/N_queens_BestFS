use crate::nqueens::NQueens;
use crate::nqueens::representations::{Tuples, ColumnVec};
use crate::nqueens::Heuristic;
use std::time::Instant;
use std::fs::File;
use std::io::Write;

pub fn test_dfs(nmax: usize, file: &mut File)
{
    for n in 4..=nmax
    {
        println!("[DFS] at n = {n}:");
        let start = Instant::now();
        let result = ColumnVec::dfs(n);
        let (solution, open_count, closed_count) = result.unwrap();
        let duration = start.elapsed();
        println!("\tColumnVec - Finished in {duration:?}");
        writeln!(file, "{};{};{};{};{:?};{};{};{}",
                 n,
                 "tuples",
                 "DFS",
                 "None",
                 solution.data,
                 open_count,
                 closed_count,
                 duration.as_secs_f64()
        ).unwrap();

        let start = Instant::now();
        let result = Tuples::dfs(n);
        let (solution, open_count, closed_count) = result.unwrap();
        let duration = start.elapsed();
        println!("\tTuples - Finished in {duration:?}");
        writeln!(file, "{};{};{};{};{:?};{};{};{}",
                 n,
                 "tuples",
                 "DFS",
                 "None",
                 solution.data,
                 open_count,
                 closed_count,
                 duration.as_secs_f64()
        ).unwrap();
        file.flush().unwrap();
    }
}