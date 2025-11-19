mod nqueens;
mod tests;

use std::time::Instant;
use nqueens::{NQueens, representations::{Tuples, ColumnVec}};

fn main()
{
    let heuristic = nqueens::Heuristic::Manhattan;
    let n = 9;
    let debug = false;
    let mut start = Instant::now();
    // println!("{:?}", ColumnVec::dfs(n));
    // println!("{:?}", ColumnVec::bfs(n));
    println!("{:?}", ColumnVec::best_fs(n, heuristic, debug));
    let mut duration = start.elapsed();
    println!("ColumnVec finished in {:?}", duration);
    start = Instant::now();
    // println!("{:?}", Tuples::dfs(n));
    // println!("{:?}", Tuples::bfs(n));
    println!("{:?}", Tuples::best_fs(n, heuristic, debug));
    duration = start.elapsed();
    println!("Tuples finished in {:?}", duration);
}