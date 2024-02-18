mod comps;
mod events;
mod resources;

pub use comps::*;
pub use events::*;
pub use resources::*;

#[derive(Clone, Copy, PartialEq)]
pub enum SoilState {
    UnTilled,
    Tilled,
    //Watered,
}
