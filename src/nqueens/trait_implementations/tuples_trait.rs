use crate::nqueens::trait_def::NQueens;
use crate::nqueens::representations::Tuples;
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

    fn create_empty(n: usize) -> Tuples
    {
        Tuples::init_empty(n, None)
    }

    fn name(&self) -> &str { "board_tuple" }

    fn generate_children(&self, n: Option<usize>) -> Vec<Tuples>
    {
        let n = n.expect("Parameter 'n' must be provided!");
        let mut children = Vec::new();
        let mut child = Tuples::init_empty(n, None);
        child.data = self.data.clone();
        child.data.push((-1, -1));
        for i in 0..n
        {
            for j in 0..n
            {
                *child.data.last_mut().unwrap() = (i as i32, j as i32);
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
        self.data.len()
    }
}