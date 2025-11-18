mod nqueens;
use std::time::Instant;
use nqueens::{NQueens, representations::{Tuples, ColumnVec}};

fn main()
{
    let n = 35;
    let mut start = Instant::now();
    // // println!("{:?}", ColumnVec::dfs(n));
    // // println!("{:?}", ColumnVec::bfs(n));
    println!("{:?}", ColumnVec::best_fs(n, nqueens::HeuristicType::AttacksCount));
    let mut duration = start.elapsed();
    println!("ColumnVec finished in {:?}", duration);

    start = Instant::now();
    // println!("{:?}", Tuples::dfs(n));
    // println!("{:?}", Tuples::bfs(n));
    println!("{:?}", Tuples::best_fs(n, nqueens::HeuristicType::AttacksCount));
    duration = start.elapsed();
    println!("Tuples finished in {:?}", duration);
}