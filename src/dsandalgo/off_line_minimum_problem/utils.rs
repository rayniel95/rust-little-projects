#[derive(Debug, Clone, Copy)]
pub enum SecuenceItem{
    E,
    I(u32)
}

pub type Sequence = Vec<SecuenceItem>;