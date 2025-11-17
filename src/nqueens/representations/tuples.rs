use crate::nqueens::enum_def::HeuristicType;
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Tuples
{
    pub data: Vec<(i32, i32)>,
    pub heuristic_type: HeuristicType,
    /// Value of the heuristic function used by Best first search
    heuristic_val: usize,
    /// Size of the chessboard
    n: usize,
}

#[allow(dead_code)]
impl Tuples
{
    pub fn init_empty(n: usize, heuristic_type: Option<HeuristicType>) -> Self
    {
        Tuples
        {
            data: Vec::new(),
            heuristic_type: heuristic_type.unwrap_or(HeuristicType::None),
            heuristic_val: 0,
            n,
        }
    }
    pub fn get_n(&self) -> usize
    {
        self.n
    }
    pub fn get_heuristic_val(&self) -> usize
    {
        self.heuristic_val
    }
    pub fn calc_heuristic(&self)
    {
        todo!()
    }
}