
#[derive(Clone)]
pub struct LinkFrom {
    pub from: u32,
    pub weight: f64,
}

#[derive(PartialEq, Eq, Clone)]
pub struct LinkTo {
    pub to: u32,
}
