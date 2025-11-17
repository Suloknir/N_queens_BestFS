#[allow(dead_code)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum HeuristicType
{
    AttacksCount,
    AttacksCountAndQueensCount,
    // Manhattan,
    None,
}