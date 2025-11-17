use std::collections::{HashSet, VecDeque};
#[allow(dead_code)]
pub trait NQueens : Sized + Eq + std::hash::Hash + Clone
{
    fn conflicts(&self) -> bool;
    fn generate_board(&mut self, n: usize);
    fn create_empty(n: usize) -> Self;
    fn name(&self) -> &str;
    fn generate_children(&self, n: Option<usize>) -> Vec<Self>;
    fn get_queens_count(&self) -> usize;
    /// Finds DFS solution of n-queens problem for given size `n`
    ///
    /// Returns `Some((T, o, c))` where:
    /// - `T` is the found DFS solution of type `T`,
    /// - `o` is the number of elements remaining in the open set
    ///   at the moment the search stopped,
    /// - `c` is the number of elements in the closed set at the
    ///    moment the search stopped,
    /// - `None` - if no solution was found.
    fn dfs(n: usize) -> Option<(Self, usize, usize)>
    {
        let mut closed = HashSet::<Self>::new();
        let mut open = Vec::<Self>::new();
        // Speeds up DFS ~ 5-10% faster
        let mut open_set = HashSet::<Self>::new();
        open.push(Self::create_empty(n));
        open_set.insert(open[0].clone());
        while let Some(current) = open.pop()
        {
            open_set.remove(&current);
            let queens_count = current.get_queens_count();
            if queens_count == n
            {
                return Some((current, open.len(), closed.len()));
            }
            let children = current.generate_children(Some(n));
            for child in children
            {
                if !closed.contains(&child) && !open_set.contains(&child)
                {
                    open.push(child.clone());
                    open_set.insert(child);
                }
            }
            closed.insert(current);
        }
        None
    }
    /// Finds BFS solution of n-queens problem for given size `n`
    ///
    /// Returns `Some((T, o, c))` where:
    /// - `T` is the found BFS solution of type `T`,
    /// - `o` is the number of elements remaining in the open set
    ///   at the moment the search stopped,
    /// - `c` is the number of elements in the closed set at the
    ///    moment the search stopped,
    /// - `None` - if no solution was found.
    fn bfs(n: usize) -> Option<(Self, usize, usize)>
    {
        let mut closed = HashSet::<Self>::new();
        let mut open = VecDeque::<Self>::new();
        // Speeds up BFS ~ 10 times faster
        let mut open_set = HashSet::<Self>::new();
        open.push_back(Self::create_empty(n));
        open_set.insert(open[0].clone());
        while let Some(current) = open.pop_front()
        {
            open_set.remove(&current);
            let queens_count = current.get_queens_count();
            if queens_count == n
            {
                return Some((current, open.len(), closed.len()));
            }
            let children = current.generate_children(Some(n));
            for child in children
            {
                if !closed.contains(&child) && !open_set.contains(&child)
                {
                    open.push_back(child.clone());
                    open_set.insert(child);
                }
            }
            closed.insert(current);
        }
        None
    }
    /// Finds solution of n-queens problem for given size `n`. Uses heuristic
    /// function defined in the struct that implements this the trait
    ///
    /// Returns `Some((T, o, c))` where:
    /// - `T` is the found BFS solution of type `T`,
    /// - `o` is the number of elements remaining in the open set
    ///   at the moment the search stopped,
    /// - `c` is the number of elements in the closed set at the
    ///    moment the search stopped,
    /// - `None` - if no solution was found.
    fn best_fs(_n: usize) -> Option<(Self, usize, usize)>
    {
        todo!()
    }
}
