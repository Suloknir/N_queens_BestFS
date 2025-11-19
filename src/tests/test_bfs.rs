use crate::nqueens::NQueens;
use crate::nqueens::representations::{Tuples, ColumnVec};
use std::time::Instant;
use std::fs::File;
use std::io::Write;

pub fn test_bfs(nmax: usize, file: &mut File)
{
    for n in 4..=nmax
    {
        println!("[BFS] at n = {n}:");
        let start = Instant::now();
        let result = ColumnVec::bfs(n);
        let (solution, open_count, closed_count) = result.unwrap();
        let duration = start.elapsed();
        println!("\tColumnVec - Finished in {duration:?}");
        writeln!(file, "{};{};{};{};{};{:?};{};{};{}",
                 n,
                 "ColumnVec",
                 "BFS",
                 "None",
                 "None",
                 solution.data,
                 open_count,
                 closed_count,
                 duration.as_secs_f64()
        ).unwrap();

        let start = Instant::now();
        let result = Tuples::bfs(n);
        let (solution, open_count, closed_count) = result.unwrap();
        let duration = start.elapsed();
        println!("\tTuples - Finished in {duration:?}");
        writeln!(file, "{};{};{};{};{};{:?};{};{};{}",
                 n,
                 "Tuples",
                 "BFS",
                 "None",
                 "None",
                 solution.data,
                 open_count,
                 closed_count,
                 duration.as_secs_f64()
        ).unwrap();
        file.flush().unwrap();
    }
    println!();
}