#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub struct RuntimeGlobals(u128);

bitflags::bitflags! {
    impl RuntimeGlobals: u128 {
    /**
     * the internal module object
     */
    const MODULE = 1 << 3;
    }
}
