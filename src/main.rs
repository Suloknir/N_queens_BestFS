mod nqueens;
use std::time::Instant;
use nqueens::{NQueens, representations::{Tuples, ColumnVec}};

fn main()
{
    let n = 8;
    let mut start = Instant::now();
    println!("{:?}", Tuples::dfs(n));
    // println!("{:?}", Tuples::bfs(n));
    let mut duration = start.elapsed();
    println!("Tuples finished in {:?}", duration);

    // start = Instant::now();
    // println!("{:?}", ColumnVec::dfs(n));
    // // println!("{:?}", ColumnVec::bfs(n));
    // duration = start.elapsed();
    // println!("ColumnVec finished in {:?}", duration);
}