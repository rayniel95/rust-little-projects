#[derive(Debug, Clone, Copy)]
pub enum SequenceItem{
    E,
    I(u32)
}

pub type Sequence = Vec<SequenceItem>;