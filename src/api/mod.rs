pub use data::OHLC;
pub use model::ChartBuffer;
pub use OHLCRenderOptions;

pub trait RendererExtension {
	fn apply(&self, _config: &OHLCRenderOptions, _buffer: &mut ChartBuffer, _data: &[OHLC]);

	fn name(&self) -> String;
}
