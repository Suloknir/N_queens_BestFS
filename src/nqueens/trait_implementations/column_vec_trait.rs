use crate::nqueens::trait_def::NQueens;
use crate::nqueens::representations::ColumnVec;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::cmp::Ordering;
use rand::Rng;

impl NQueens for ColumnVec
{
    fn conflicts(&self) -> bool
    {
        let mut seen = HashSet::new();
        for x in self.data.iter().filter(|&&x| x >= 0)
        {
            if !seen.insert(x)
            {
                return true;
            }
        }
        for (c1, r1) in self.data.iter().enumerate().filter(|&(_, &x)| x >= 0)
        {
            for (c2, r2) in self.data.iter().enumerate().skip(c1 + 1).filter(|&(_, &x)| x >= 0)
            {
                if (r1 - r2).abs() == (c1 as i32 - c2 as i32).abs()
                {
                    return true;
                }
            }
        }
        false
    }
    fn generate_board(&mut self, n: usize)
    {
        let mut rng = rand::rng();
        let board: Vec<i32> = (0..n)
            .map(|_| rng.random_range(0..n as i32))
            .collect();
        self.data = board;
    }

    fn create_empty(n: usize) -> ColumnVec
    {
        ColumnVec::init_empty(n, None)
    }

    fn name(&self) -> &str { "board_vector" }

    fn generate_children(&self, n: Option<usize>) -> Vec<ColumnVec>
    {
        let n = n.unwrap_or(self.data.len());
        if self.queens_count >= n
        {
            return Vec::new();
        }
        let mut children = Vec::new();
        // let mut child = self.clone();
        for col in 0..n
        {
            let mut child = ColumnVec::init_empty(n, None);
            child.data = self.data.clone();
            child.data[self.queens_count] = col as i32;
            child.queens_count = self.queens_count + 1;
            // if !child.conflicts() {children.push(child.clone());}
            children.push(child);
        }
        children
    }

    fn get_queens_count(&self) -> usize
    {
        self.queens_count
    }
}

impl PartialEq for ColumnVec
{
    fn eq(&self, other: &Self) -> bool
    {
        self.data == other.data
    }
}

impl Eq for ColumnVec{}

impl Hash for ColumnVec
{
    fn hash<H: Hasher>(&self, state: &mut H)
    {
        self.data.hash(state);
    }
}

impl PartialOrd for ColumnVec
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        Some(self.cmp(other))
    }
}

impl Ord for ColumnVec
{
    //Reversed ordering, heuristic_val = 0 > heuristic_val = 2
    fn cmp(&self, other: &Self) -> Ordering
    {
        other.get_heuristic_val().cmp(&self.get_heuristic_val())
    }
}