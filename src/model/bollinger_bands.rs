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
	fn apply(&self, config: &OHLCRenderOptions, buffer: &mut ChartBuffer, data: &[OHLC]) {}
}

fn std_dev(prices: &[f64]) -> f64 {
	let avg = avg(prices);
	let mut squared_diff_sum = 0.;

	for price in prices {
		squared_diff_sum += (avg - price).pow(2);
	}

	(squared_diff_sum / (prices.len() - 1)).sqrt()
}

fn avg(prices: &[f64]) -> f64 {
	let mut sum = 0.;

	for price in prices {
		sum += price;
	}

	sum / prices.len()
}

fn middle_of_ohlc(ohlc: OHLC) -> f64 {
	(ohlc.h - ohlc.l) / 2 + ohlc.l
}
