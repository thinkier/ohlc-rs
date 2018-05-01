use api::RendererExtension;
use data::OHLC;
use model::ChartBuffer;

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
			let mut price = buffer.min_price;
			while price <= buffer.max_price {
				let p1 = buffer.data_to_coords(price, 0);
				let p2 = buffer.data_to_coords(price, buffer.timeframe);
				buffer.line(p1, p2, self.colour);

				price += self.price_interval;
			}
		}

		{
			let mut time = 0;
			while time <= buffer.timeframe {
				let p1 = buffer.data_to_coords(buffer.min_price, time);
				let p2 = buffer.data_to_coords(buffer.max_price, time);
				buffer.line(p1, p2, self.colour);

				time += self.time_interval;
			}
		}
	}

	fn name(&self) -> String {
		"CORE_GridLines()".to_string()
	}
}
