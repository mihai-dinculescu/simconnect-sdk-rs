#[derive(Debug, Clone)]
pub enum PeriodEnum {
    VisualFrame { interval: u32 },
    Second,
}
