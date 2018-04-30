use api::RendererExtension;
use data::OHLC;
use model::ChartBuffer;
use OHLCRenderOptions;

pub struct BollingerBand {
	periods: usize,
	standard_deviations: usize,
}

impl BollingerBand {
	pub fn new(periods: usize, standard_deviations: usize) -> BollingerBand {
		BollingerBand { periods, standard_deviations }
	}
}

impl RendererExtension for BollingerBand {
	fn apply(&self, config: &OHLCRenderOptions, buffer: &mut ChartBuffer, data: &[OHLC]) {

	}
}
