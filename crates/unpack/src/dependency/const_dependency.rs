use super::DependencyTemplate;

#[derive(Debug,Clone)]
pub struct ConstDependency {
    pub start: u32,
    pub end: u32,
    pub content: String,
}
impl ConstDependency {
    pub fn new(start:u32, end:u32, content: String) -> Self{
        Self {
            start,
            end,
            content
        }
    }

}

impl DependencyTemplate for ConstDependency {
    fn apply(&self, source:&mut rspack_sources::ReplaceSource<rspack_sources::BoxSource>, _code_generation_context: &crate::module::CodeGenerationContext ) {
        source.replace(self.start, self.end, self.content.as_ref(), None);
    }
}