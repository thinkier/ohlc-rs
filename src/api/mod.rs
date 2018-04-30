pub use data::OHLC;
pub use model::ChartBuffer;
pub use OHLCRenderOptions;
use std::fmt::Debug;


pub trait RendererExtension: 'static + Debug {
	fn apply(&self, _config: &OHLCRenderOptions, _buffer: &mut ChartBuffer, _data: &[OHLC]);

	fn name(&self) -> String;
}

impl PartialEq for RendererExtension{
	fn eq(&self, other: &RendererExtension) -> bool {
		self.name() == other.name()
	}
}