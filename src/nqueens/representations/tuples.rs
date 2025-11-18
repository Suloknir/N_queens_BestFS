use crate::nqueens::enum_def::HeuristicType;
#[derive(Clone, Debug)]
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
        for (i, (r1, c1)) in self.data.iter().enumerate() //.filter(|&(_, (r, c))| *r >= 0 && *c>=0)
        {
            for (r2, c2) in self.data.iter().skip(i + 1)
            {
                if r1 == r2 ||
                    c1 == c2 ||
                    (r1 - r2).abs() == (c1 - c2).abs()
                {
                    result += 1;
                }
            }
        }
        result
    }
}