#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Tuples
{
    pub data: Vec<(i32, i32)>,
    n: usize,
}

#[allow(dead_code)]
impl Tuples
{
    pub fn init_empty(n: usize) -> Self
    {
        Tuples
        {
            data: Vec::new(),
            n,
        }
    }
    pub fn get_n(&self) -> usize
    {
        self.n
    }
}