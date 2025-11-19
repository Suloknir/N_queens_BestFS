use crate::nqueens::NQueens;
use crate::nqueens::representations::{Tuples, ColumnVec};
use crate::nqueens::Heuristic;
use std::time::Instant;
use std::fs::File;
use std::io::Write;

pub fn test_bestfs(nmax: &[usize], file: &mut File)
{
    for (i, &heuristic) in Heuristic::all_variants().iter().enumerate()
    {
        for n in 4..=nmax[i]
        {
            println!("[BestFS] at n = {n}, heuristic = {heuristic:?}:");
            let start = Instant::now();
            let result = ColumnVec::best_fs(n, heuristic, false);
            let (solution, open_count, closed_count) = result.unwrap();
            let duration = start.elapsed();
            println!("\tColumnVec - Finished in {duration:?}");
            writeln!(file, "{};{};{};{:?};{};{:?};{};{};{}",
                     n,
                     "ColumnVec",
                     "BestFS",
                     heuristic,
                     solution.get_heuristic_val(),
                     solution.data,
                     open_count,
                     closed_count,
                     duration.as_secs_f64()
            ).unwrap();

            let start = Instant::now();
            let result = Tuples::best_fs(n, heuristic, false);
            let (solution, open_count, closed_count) = result.unwrap();
            let duration = start.elapsed();
            println!("\tTuples - Finished in {duration:?}");
            writeln!(file, "{};{};{};{:?};{};{:?};{};{};{}",
                     n,
                     "Tuples",
                     "BestFS",
                     heuristic,
                     solution.get_heuristic_val(),
                     solution.data,
                     open_count,
                     closed_count,
                     duration.as_secs_f64()
            ).unwrap();
            file.flush().unwrap();
        }
        println!();
    }
}