pub use buffer::*;
pub use data::OHLC;
pub use painting::*;
use std::fmt::Debug;

pub mod buffer;
pub mod painting;
pub mod rex;

pub trait RendererExtension: Debug {
	fn apply(&self, _buffer: &mut ChartBuffer, _data: &[OHLC]);

	fn name(&self) -> String;
}

pub struct Margin {
	pub top: usize,
	pub bottom: usize,
	pub left: usize,
	pub right: usize,
}
