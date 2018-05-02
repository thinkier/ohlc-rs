use api::RendererExtension;
use data::OHLC;
use model::ChartBuffer;
use utils::duration_string;

#[derive(Debug)]
pub struct GridLines {
	colour: u32,
	label: bool,
	price_interval: f64,
	time_interval: i64,
}

impl GridLines {
	pub fn new(colour: u32, label: bool, price_interval: f64, time_interval: i64) -> GridLines {
		GridLines { colour, label, price_interval, time_interval }
	}
}

impl RendererExtension for GridLines {
	fn apply(&self, buffer: &mut ChartBuffer, _data: &[OHLC]) {
		{
			let mut price = round_start_price(&buffer, self.price_interval);
			while price <= buffer.max_price {
				let p1 = buffer.data_to_coords(price, 0);
				let p2 = buffer.data_to_coords(price, buffer.timeframe);
				buffer.line(p1, p2, self.colour);
				if self.label {
					buffer.text((p2.0 + 4, p2.1 - 8), &format!("{:.8}", price), self.colour);
				}

				price += self.price_interval;
			}
		}

		{
			let mut time = 0;
			while time <= buffer.timeframe {
				let p1 = {
					let point = buffer.data_to_coords(buffer.min_price, time);
					(point.0, point.1 + 15)
				};
				let p2 = buffer.data_to_coords(buffer.max_price, time);

				buffer.line(p1, p2, self.colour);

				if self.label {
					let elapsed = format!("{}", duration_string((buffer.timeframe - time) as u64));
					buffer.text((p1.0 - 10, p1.1 + 2), &elapsed, self.colour);
				}

				time += self.time_interval;
			}
		}
	}

	fn name(&self) -> String {
		"CORE_GridLines()".to_string()
	}
}

fn round_start_price(buffer: &ChartBuffer, interval: f64) -> f64 {
	let window = buffer.max_price - buffer.min_price;
	let count = (window / interval).floor();

	buffer.min_price + interval - (buffer.min_price % interval)
}
