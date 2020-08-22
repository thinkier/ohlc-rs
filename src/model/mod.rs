pub use buffer::*;
pub use data::OHLC;
pub use painting::*;

pub use self::rex::RendererExtension;

pub mod buffer;
pub mod painting;
pub mod rex;

pub struct Margin {
    pub top: usize,
    pub bottom: usize,
    pub left: usize,
    pub right: usize,
}
