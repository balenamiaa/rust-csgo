#[allow(conflicting_repr_hints)]
#[repr(C, i32)]
#[derive(Copy, Clone, PartialEq)]
pub enum EntityTeam {
    CT = 3,
    T = 2,
    SPECTATOR = 1,
    NONE = 0,
}
