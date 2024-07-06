use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum Population {
    ThinDisc(Subpopulation),
    ThickDisc(Subpopulation),
    Spheroid,
    Bulge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum Subpopulation {
    Alive,
    WhiteDwarf,
}
