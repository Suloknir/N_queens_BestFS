#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ColumnVec
{
    pub data: Vec<i32>,
    pub queens_count: usize,
    n: usize,
}

#[allow(dead_code)]
impl ColumnVec
{
    pub fn init_empty(n: usize) -> Self
    {
        ColumnVec
        {
            data: vec![-1; n],
            queens_count: 0,
            n,
        }
    }
    pub fn get_n(&self) -> usize
    {
        self.n
    }
}