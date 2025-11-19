#[allow(dead_code)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Heuristic
{
    AttacksCount,
    AttacksCountAndQueensCount,
    Manhattan,
    None,
}