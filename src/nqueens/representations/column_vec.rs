use crate::nqueens::enum_def::Heuristic;
#[derive(Clone, Debug)]
pub struct ColumnVec
{
    pub data: Vec<i32>,
    pub heuristic_type: Heuristic,
    /// Value of the heuristic function used by Best first search
    heuristic_val: usize,
    /// Size of the chessboard
    n: usize,
}

#[allow(dead_code)]
impl ColumnVec
{
    pub fn init_empty(n: usize, heuristic_type: Option<Heuristic>) -> Self
    {
        ColumnVec
        {
            data: Vec::new(),
            heuristic_type: heuristic_type.unwrap_or(Heuristic::None),
            heuristic_val: n,
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
            Heuristic::AttacksCount =>
            {
                self.heuristic_val = self.attacks_count();
            }
            Heuristic::AttacksCountAndQueensCount =>
            {
                let queens_count = self.data.len();
                self.heuristic_val = self.attacks_count() + (self.n - queens_count);
            }
            Heuristic::Manhattan =>
            {
                self.heuristic_val =
                {
                    let mut result = 0;
                    let queens_count = self.data.len();
                    for c1 in 0..queens_count
                    {
                        let r1 = self.data[c1];
                        for c2 in (c1 + 1)..queens_count
                        {
                            let r2 = self.data[c2];
                            result +=
                                {
                                    let distance = (r2 - r1).abs() as usize + (c2 as i32- c1 as i32).abs() as usize;
                                    if distance <= 3
                                    {
                                        distance
                                    }
                                    else
                                    {
                                        distance * self.n
                                    }
                                }
                        }
                    }
                    result
                }
            }
            Heuristic::None =>
            {
                self.heuristic_val = self.n;
            }
        }
    }

    fn attacks_count(&self) -> usize
    {
        let mut result = 0;
        let queens_count = self.data.len();
        for i in 0..queens_count
        {
            let r1 = self.data[i];
            for j in (i + 1)..queens_count
            {
                let r2 = self.data[j];
                if r1 == r2
                {
                    result += 1;
                }
                if (r1 - r2).abs() == (j - i) as i32
                {
                    result += 1;
                }
            }
        }
        result
    }
}