use api::RendererExtension;
use data::OHLC;
use model::ChartBuffer;
use utils::*;

#[derive(Debug)]
pub struct BasicIndicativeLines {
	max_colour: u32,
	min_colour: u32,
	current_colour: u32,
}

impl BasicIndicativeLines {
	pub fn new(max_colour: u32, min_colour: u32, current_colour: u32) -> BasicIndicativeLines {
		BasicIndicativeLines { max_colour, min_colour, current_colour }
	}
}

impl RendererExtension for BasicIndicativeLines {
	fn apply(&self, buffer: &mut ChartBuffer, data: &[OHLC]) {
		let data = calculate_ohlc_of_set(data);

		draw(buffer, data.h, self.max_colour);
		draw(buffer, data.l, self.min_colour);
		draw(buffer, data.c, self.current_colour);
	}

	fn name(&self) -> String {
		"CORE_BasicIndicativeLines()".to_string()
	}
}

fn draw(buffer: &mut ChartBuffer, price: f64, rgba: u32) {
	let p1 = buffer.data_to_coords(price, 0);
	let p2 = buffer.data_to_coords(price, buffer.timeframe);

	buffer.line(p1, p2, rgba);
	buffer.text_with_outline((p2.0 + 3, p2.1 - 9), &format!("{:.8}", price), rgba);
}
