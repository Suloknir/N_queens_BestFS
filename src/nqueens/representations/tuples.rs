use crate::nqueens::enum_def::Heuristic;
#[derive(Clone, Debug)]
pub struct Tuples
{
    pub data: Vec<(i32, i32)>,
    pub heuristic_type: Heuristic,
    /// Value of the heuristic function used by Best first search
    heuristic_val: usize,
    /// Size of the chessboard
    n: usize,
}

#[allow(dead_code)]
impl Tuples
{
    pub fn init_empty(n: usize, heuristic_type: Option<Heuristic>) -> Self
    {
        Tuples
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
                    for (i, &(r1, c1)) in self.data.iter().enumerate() //.filter(|&(_, (r, c))| *r >= 0 && *c>=0)
                    {
                        for &(r2, c2) in self.data.iter().skip(i + 1)
                        {
                            result +=
                            {
                                let distance = Self::manhattan_distance((r1, c1), (r2, c2));
                                if distance <= 3
                                {
                                    distance * self.n
                                }
                                else
                                {
                                    distance
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
    fn manhattan_distance(q1: (i32, i32), q2: (i32, i32)) -> usize
    {
        let (r1, c1) = q1;
        let (r2, c2) = q2;
        (r1 - r2).abs() as usize - (c1 - c2).abs() as usize
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