mod nqueens;
mod tests;
use std::time::Instant;

fn main()
{
    let start = Instant::now();
    let nmax_dfs = 9;
    let nmax_bfs = 7;
    let nmax_h1: usize = 45; //nmax for attacks count
    let nmax_h2: usize = 11; //nmax for attacks count and queens count
    let nmax_h3: usize = 9; //nmax for manhattan
    let nmax_bestfs = Vec::from(&[nmax_h1, nmax_h2, nmax_h3]);
    tests::experiment::test(nmax_dfs, nmax_bfs, &nmax_bestfs);
    let duration = start.elapsed();
    println!();
    println!("Finished in {:?}", duration);
}