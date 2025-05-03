#[derive(Debug, Clone, Copy)]
pub struct DependencyRange {
    pub start: u32,
    pub end: u32,
}

impl DependencyRange {
    fn new(start: u32, end: u32) -> Self {
        DependencyRange { start, end }
    }
}

impl From<swc_core::common::Span> for DependencyRange {
    fn from(span: swc_core::common::Span) -> Self {
        Self {
            start: span.lo.0.saturating_sub(1),
            end: span.hi.0.saturating_sub(1),
        }
    }
}
