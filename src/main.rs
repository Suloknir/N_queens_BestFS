mod nqueens;
use std::time::Instant;
use nqueens::{NQueens, representations::{Tuples, ColumnVec}};
fn main()
{
    let start = Instant::now();
    println!("{:?}", Tuples::dfs(11));
    println!("{:?}", Tuples::bfs(6));
    println!("{:?}", ColumnVec::dfs(12));
    println!("{:?}", ColumnVec::bfs(7));
    let duration = start.elapsed();
    println!("Finished in {:?}", duration);
}