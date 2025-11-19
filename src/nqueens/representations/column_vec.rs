use crate::nqueens::enum_def::HeuristicType;
#[derive(Clone, Debug)]
pub struct ColumnVec
{
    pub data: Vec<i32>,
    pub heuristic_type: HeuristicType,
    /// Value of the heuristic function used by Best first search
    heuristic_val: usize,
    /// Size of the chessboard
    n: usize,
}

#[allow(dead_code)]
impl ColumnVec
{
    pub fn init_empty(n: usize, heuristic_type: Option<HeuristicType>) -> Self
    {
        ColumnVec
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

    pub fn calc_heuristic(&mut self)
    {
        match self.heuristic_type
        {
            HeuristicType::AttacksCount =>
            {
                self.heuristic_val = self.attacks_count();
            }
            HeuristicType::AttacksCountAndQueensCount =>
            {
                let queens_count = self.data.len();
                self.heuristic_val = self.attacks_count() + (self.n - queens_count);
            }
            HeuristicType::None =>
            {
                self.heuristic_val = 0;
            }
        }
    }

    fn attacks_count(&self) -> usize
    {
        let mut result = 0;
        for (c1, r1) in self.data.iter().enumerate() //.filter(|&(_, &x)| x >= 0)
        {
            for (c2, r2) in self.data.iter().enumerate().skip(c1 + 1) //.filter(|&(_, &x)| x >= 0)
            {
                if r1 == r2
                {
                    result += 1;
                }
                if (r1 - r2).abs() == (c1 as i32 - c2 as i32).abs()
                {
                    result += 1;
                }
            }
        }
        result
    }
}