use crate::nqueens::trait_def::NQueens;
use crate::nqueens::representations::ColumnVec;
use std::collections::HashSet;
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
        ColumnVec::init_empty(n)
    }

    fn name(&self) -> &str { "board_vector" }

    fn generate_children(&self, n: Option<usize>) -> Vec<ColumnVec>
    {
        let n = n.unwrap_or(self.data.len());
        let mut children = Vec::new();
        for (id, _) in self.data.iter().enumerate().filter(|&(_, &x)| x < 0)
        {
            let mut child = self.clone();
            for i in 0..n
            {
                child.data[id] = i as i32;
                child.queens_count = self.queens_count + 1;
                if !child.conflicts()
                {
                    children.push(child.clone());
                }
            }
        }
        children
    }

    fn get_queens_count(&self) -> usize
    {
        self.queens_count
    }
}
