use crate::tests::{test_dfs, test_bfs, test_bestfs};
use std::fs::File;
use std::io::Write;

pub fn test(nmax_dfs: usize, nmax_bfs: usize, nmax_bestfs: &[usize])
{
    let mut file = File::create("results.csv").unwrap();
    writeln!(file, "N;Representation;Algorithm;Heuristic;Heuristic_value;Solution;Open_count;Closed_count;Execution_time(s)").unwrap();
    test_dfs(nmax_dfs, &mut file);
    test_bfs(nmax_bfs, &mut file);
    test_bestfs(nmax_bestfs, &mut file);
    file.flush().unwrap();
}