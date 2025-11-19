use std::collections::{HashSet, VecDeque, BinaryHeap};
use std::cmp::Reverse;
use crate::nqueens::enum_def::Heuristic;
#[allow(dead_code)]
pub trait NQueens : Sized + Eq + std::hash::Hash + Clone + Ord + std::fmt::Debug
{
    fn conflicts(&self) -> bool;
    fn generate_board(&mut self, n: usize);
    fn create_empty(n: usize, heuristic_type: Option<Heuristic>) -> Self;
    fn name(&self) -> &str;
    fn generate_children(&self, n: usize) -> Vec<Self>;
    fn calc_heuristic(&mut self);
    fn get_heuristic_val(&self) -> usize;
    fn get_queens_count(&self) -> usize;
    /// Finds DFS solution of n-queens problem for given size `n`.
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
        // let mut open_set = HashSet::<Self>::new();
        open.push(Self::create_empty(n, None));
        // open_set.insert(open[0].clone());
        while let Some(current) = open.pop()
        {
            // open_set.remove(&current);
            let queens_count = current.get_queens_count();
            if queens_count == n && !current.conflicts()
            {
                return Some((current, open.len(), closed.len()));
            }
            let children = current.generate_children(n);
            // assert!(children.len() <= n);
            for child in children
            {
                if !closed.contains(&child) && !open.contains(&child)
                {
                    open.push(child.clone());
                    // open_set.insert(child);
                }
            }
            closed.insert(current);
        }
        None
    }
    /// Finds BFS solution of n-queens problem for given size `n`.
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
        // Speeds up BFS ~ 10-200 times faster
        let mut open_set = HashSet::<Self>::new();
        open.push_back(Self::create_empty(n, None));
        open_set.insert(open[0].clone());
        while let Some(current) = open.pop_front()
        {
            open_set.remove(&current);
            let queens_count = current.get_queens_count();
            if queens_count == n && !current.conflicts()
            {
                return Some((current, open.len(), closed.len()));
            }
            let children = current.generate_children(n);
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
    /// function defined in the struct that implements this trait to determine
    /// which nodes should be expanded first.
    ///
    /// Returns `Some((T, o, c))` where:
    /// - `T` is the found BFS solution of type `T`,
    /// - `o` is the number of elements remaining in the open set
    ///   at the moment the search stopped,
    /// - `c` is the number of elements in the closed set at the
    ///    moment the search stopped,
    /// - `None` - if no solution was found.
    fn best_fs(n: usize, heuristic_type: Heuristic, debug: bool) -> Option<(Self, usize, usize)>
    {
        let mut closed = HashSet::<Self>::new();
        let mut open_min_heap = BinaryHeap::<Reverse<Self>>::new();
        let mut open_set = HashSet::<Self>::new();
        let mut state0 = Self::create_empty(n, Some(heuristic_type));
        state0.calc_heuristic();
        // println!("{state0:?}");
        open_set.insert(state0.clone());
        open_min_heap.push(Reverse(state0));
        while let Some(Reverse(current)) = open_min_heap.pop()
        {
            open_set.remove(&current);
            if debug
            {
                println!("{current:?}");
            }
            let queens_count = current.get_queens_count();
            if queens_count == n && !current.conflicts()
            {
                return Some((current, open_min_heap.len(), closed.len()));
            }
            let children = current.generate_children(n);
            for mut child in children
            {
                if closed.contains(&child)
                {
                    continue;
                }
                child.calc_heuristic();
                if !open_set.contains(&child)
                {
                    open_min_heap.push(Reverse(child.clone()));
                    open_set.insert(child);
                }
                else
                {
                    // if child.get_heuristic_val() < open_set.get(&child)?.get_heuristic_val()
                    if child.cmp(open_set.get(&child)?) == std::cmp::Ordering::Less
                    {
                        open_set.replace(child.clone());
                        //Todo: figure out how to delete duplicate from binaryheap to save memory
                        open_min_heap.push(Reverse(child));
                    }
                }
            }
            closed.insert(current);
        }
        None
    }
}
