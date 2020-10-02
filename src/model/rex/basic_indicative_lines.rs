use serde::export::PhantomData;

use model::*;
use utils::*;

#[derive(Clone, Debug)]
pub struct BasicIndicativeLines<C> {
	_c: PhantomData<C>,
	max_colour: u32,
	min_colour: u32,
	current_colour: u32,
}

impl<C> BasicIndicativeLines<C> {
	pub fn new(max_colour: u32, min_colour: u32, current_colour: u32) -> BasicIndicativeLines<C> {
		BasicIndicativeLines { _c: PhantomData, max_colour, min_colour, current_colour }
	}
}

impl<C: Candle> RendererExtension for BasicIndicativeLines<C> {
	type Candle = C;

	fn apply(&self, buffer: &mut ChartBuffer, data: &[C]) {
		let data = aggregate(data);

		draw(buffer, data.h, self.max_colour);
		draw(buffer, data.l, self.min_colour);
		draw(buffer, data.c, self.current_colour);
	}

	fn lore_colour(&self) -> Option<u32> {
		None
	}

	fn name(&self) -> String {
		"CORE_BasicIndicativeLines()".to_string()
	}
}

fn draw(buffer: &mut ChartBuffer, price: f64, rgba: u32) {
	let p1 = buffer.data_to_coords(price, 0);
	let p2 = buffer.data_to_coords(price, buffer.timeframe);

	buffer.line(p1, p2, rgba);
	buffer.text_with_outline((p2.0 + 3, p2.1 - 9), &format!("{:.1}", price), rgba);
}
