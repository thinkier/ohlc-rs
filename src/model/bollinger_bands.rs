use model::*;

#[derive(Debug)]
struct BandPoints {
	higher: f64,
	median: f64,
	lower: f64,
}

#[derive(Debug)]
pub struct BollingerBands {
	periods: usize,
	standard_deviations: usize,
	line_colour: u32,
}

impl BollingerBands {
	pub fn new(periods: usize, standard_deviations: usize, line_colour: u32) -> BollingerBands {
		BollingerBands { periods, standard_deviations, line_colour }
	}
}

impl RendererExtension for BollingerBands {
	fn apply(&self, buffer: &mut ChartBuffer, data: &[OHLC]) {
		let half_period = self.periods / 2;

		let mut bands = vec![];

		for i in 0..data.len() {
			let min = if i > half_period { i - half_period } else { 0 };
			let max = {
				let proto_max = i + half_period + 1;
				if proto_max >= data.len() {
					data.len() - 1
				} else {
					proto_max
				}
			};

			let data_slice = &data[min..max];
			let medians = into_median(data_slice);
			let scaled_std_dev = std_dev(&medians[..]) * self.standard_deviations as f64;
			let moving_avg = avg(&medians[..]);
			let points = BandPoints {
				higher: moving_avg + scaled_std_dev,
				median: moving_avg,
				lower: moving_avg - scaled_std_dev,
			};

			bands.push(points);
		}

		let offset = buffer.timeframe / (2 * data.len()) as i64;

		for i in 0..(bands.len() - 1) {
			let time = (i as i64 * buffer.timeframe / data.len() as i64) as i64 + offset;
			let time_next_period = ((i as i64 + 1) * buffer.timeframe / data.len() as i64) as i64 + offset;

			let p1_h = buffer.data_to_coords(bands[i].higher, time);
			let p2_h = buffer.data_to_coords(bands[i + 1].higher, time_next_period);

			buffer.line(p1_h, p2_h, self.line_colour);

			let p1_m = buffer.data_to_coords(bands[i].median, time);
			let p2_m = buffer.data_to_coords(bands[i + 1].median, time_next_period);

			buffer.line(p1_m, p2_m, self.line_colour);

			let p1_l = buffer.data_to_coords(bands[i].lower, time);
			let p2_l = buffer.data_to_coords(bands[i + 1].lower, time_next_period);

			buffer.line(p1_l, p2_l, self.line_colour);
		}
	}

	fn name(&self) -> String {
		format!("BB({}, {})", self.periods, self.standard_deviations)
	}
}

fn std_dev(prices: &[f64]) -> f64 {
	let avg = avg(prices);
	let mut squared_diff_sum = 0.;

	for price in prices {
		squared_diff_sum += (avg - price).powf(2.);
	}

	(squared_diff_sum / (prices.len() - 1) as f64).sqrt()
}

fn avg(prices: &[f64]) -> f64 {
	let mut sum = 0.;

	for price in prices {
		sum += *price;
	}

	sum / prices.len() as f64
}

fn middle_of_ohlc(ohlc: OHLC) -> f64 {
	((ohlc.h - ohlc.l) / 2.) + ohlc.l
}

fn into_median(list: &[OHLC]) -> Vec<f64> {
	let mut buffer = vec![];

	for ohlc in list {
		buffer.push(middle_of_ohlc(*ohlc));
	}

	return buffer;
}
