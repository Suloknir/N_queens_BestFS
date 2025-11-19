use crate::nqueens::trait_def::NQueens;
use crate::nqueens::representations::ColumnVec;
use crate::nqueens::enum_def::Heuristic;
use std::hash::{Hash, Hasher};
use std::cmp::Ordering;
use rand::Rng;

impl NQueens for ColumnVec
{
    fn conflicts(&self) -> bool
    {
        let queens_count = self.data.len();
        for i in 0..queens_count
        {
            let r1 = self.data[i];
            for j in (i + 1)..queens_count
            {
                let r2 = self.data[j];
                if r1 == r2
                {
                    return true;
                }
                if (r1 - r2).abs() == (j - i) as i32
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

    fn create_empty(n: usize, heuristic_type: Option<Heuristic>) -> Self
    {
        ColumnVec::init_empty(n, heuristic_type)
    }

    fn name(&self) -> &str { "board_vector" }

    fn generate_children(&self, n: usize) -> Vec<ColumnVec>
    {
        let queens_count = self.data.len();
        if queens_count >= n
        {
            return Vec::new();
        }
        let mut children = Vec::new();
        for col in 0..n
        {
            let mut child = self.clone();
            child.data.push(col as i32);
            // if !child.conflicts() {children.push(child.clone());}
            children.push(child);
        }
        children
    }

    fn calc_heuristic(&mut self)
    {
        self.calc_heuristic();
    }

    fn get_heuristic_val(&self) -> usize
    {
        self.get_heuristic_val()
    }

    fn get_queens_count(&self) -> usize
    {
        self.data.len()
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
    fn cmp(&self, other: &Self) -> Ordering
    {
        match self.heuristic_type
        {
            //Reversed ordering
            Heuristic::Manhattan => other.get_heuristic_val().cmp(&self.get_heuristic_val()),
            _ => self.get_heuristic_val().cmp(&other.get_heuristic_val())
        }
    }
}