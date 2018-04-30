use data::OHLC;
use model::ChartBuffer;
use OHLCRenderOptions;

pub trait RendererExtension {
	fn apply(&self, _config: &OHLCRenderOptions, _buffer: &mut ChartBuffer, _data: &[OHLC]);
}