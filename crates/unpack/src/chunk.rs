pub mod chunk_graph;
pub mod chunk_group;
pub mod chunk_group_id;
pub mod chunk_id;
pub mod chunk_linker;
#[derive(Debug)]
pub struct Chunk {
    pub name: Option<String>,
}

impl Chunk {
    pub fn new(name: Option<String>) -> Self {
        Self { name }
    }
}
