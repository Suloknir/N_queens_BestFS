#[allow(dead_code)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Heuristic
{
    AttacksCount,
    AttacksCountAndQueensCount,
    Manhattan,
    None,
}

impl Heuristic
{
    pub fn all_variants() -> &'static[Heuristic]
    {
        &[
            Heuristic::AttacksCount,
            Heuristic::AttacksCountAndQueensCount,
            Heuristic::Manhattan,
        ]
    }
}