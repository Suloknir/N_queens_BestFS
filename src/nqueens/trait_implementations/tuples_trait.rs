use crate::nqueens::trait_def::NQueens;
use crate::nqueens::representations::Tuples;
use crate::nqueens::enum_def::Heuristic;
use std::hash::{Hash, Hasher};
use std::cmp::Ordering;
use rand::Rng;

impl NQueens for Tuples
{
    fn conflicts(&self) -> bool
    {
        for (i, (r1, c1)) in self.data.iter().enumerate() //.filter(|&(_, (r, c))| *r >= 0 && *c>=0)
        {
            for (r2, c2) in self.data.iter().skip(i + 1)
            {
                if r1 == r2 ||
                    c1 == c2 ||
                    (r1 - r2).abs() == (c1 - c2).abs()
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
        //in this case queens can overlap each other but i leave it as it is
        let board: Vec<(i32, i32)> = (0..n)
            .map(|_| (rng.random_range(0..n as i32), rng.random_range(0..n as i32)))
            .collect();
        self.data = board;
    }

    fn create_empty(n: usize, heuristic_type: Option<Heuristic>) -> Self
    {
        Tuples::init_empty(n, heuristic_type)
    }

    fn name(&self) -> &str { "tuples" }

    fn generate_children(&self, n: usize) -> Vec<Tuples>
    {
        let queens_count = self.data.len();
        if queens_count >= n
        {
            return Vec::new();
        }
        let mut children = Vec::new();
        let next_row = self.data.len();
        for col in 0..n
        {
            let mut child = self.clone();
            child.data.push((next_row as i32, col as i32));
            // if !child.conflicts(){children.push(child.clone());}
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

impl PartialEq for Tuples
{
    fn eq(&self, other: &Self) -> bool
    {
        self.data == other.data
    }
}

impl Eq for Tuples {}

impl Hash for Tuples
{
    fn hash<H: Hasher>(&self, state: &mut H)
    {
        self.data.hash(state);
    }
}

impl PartialOrd for Tuples
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        Some(self.cmp(other))
    }
}

impl Ord for Tuples
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