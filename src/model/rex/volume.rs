use std::marker::PhantomData;

use model::*;

#[derive(Clone, Debug)]
pub struct Volume<C> {
	_c: PhantomData<C>,
	label_colour: u32,
	buy_colour: u32,
	sell_colour: u32,
	generic_colour: u32,
}

impl<C> Volume<C> {
	pub fn new(label_colour: u32, buy_colour: u32, sell_colour: u32, generic_colour: u32) -> Volume<C> {
		Volume { _c: PhantomData, label_colour, buy_colour, sell_colour, generic_colour }
	}
}

impl<C: Candle> RendererExtension for Volume<C> {
	type Candle = C;

	fn apply(&self, buffer: &mut ChartBuffer, data: &[C]) {
		let mut vols = vec![];
		let mut max_vol = 0.;

		for i in 0..data.len() {
			let candle = &data[i];
			let b_vol = candle.buy_volume();
			let total_vol = candle.total_volume();

			if total_vol > max_vol {
				max_vol = total_vol;
			}
			vols.push((b_vol, total_vol));
		}

		buffer.create_extension_strip(175, move |buffer| {
			buffer.text((8, 8), "Volume", self.label_colour);

			// Lines and labels
			{
				for prog in &[0., 0.5, 1.] {
					let p1 = buffer.data_to_coords(*prog, 0);
					let p2 = buffer.data_to_coords(*prog, buffer.timeframe);

					buffer.line(p1, p2, self.label_colour);

					let price = prog * max_vol;

					buffer.text_with_outline((p2.0 + 5, p2.1 - 9), &format!("{}", keep_msf(price, 3)), self.label_colour);
				}
			}

			// Rendering of the volume candles
			{
				let period = buffer.timeframe / data.len() as i64;
				let period_addition = 4. * period as f64 / 5.;

				for i in 0..vols.len() {
					let (b, t) = vols[i];

					let left_most = period * i as i64;
					let right_most = ((period * (i as i64)) as f64 + period_addition) as i64;

					let bottom_left = buffer.data_to_coords(0., left_most);
					if let Some(b) = b {
						let mid_right = buffer.data_to_coords(b / max_vol, right_most);

						buffer.rect_point(bottom_left, mid_right, self.buy_colour);
						let top_left = buffer.data_to_coords(t / max_vol, left_most);
						buffer.rect_point(mid_right, top_left, self.sell_colour);
					} else {
						let top_right = buffer.data_to_coords(t / max_vol, right_most);
						buffer.rect_point(bottom_left, top_right, self.generic_colour);
					}
				}
			}
		});
	}

	fn lore_colour(&self) -> Option<u32> {
		None
	}

	fn name(&self) -> String {
		"Volume".to_string()
	}
}

fn keep_msf(num: f64, sigfigs: usize) -> f64 {
	if sigfigs == 0 || num == 0. {
		return 0.;
	}

	let mag = num.log10().floor();
	let factor = mag - sigfigs as f64 + 1.;

	(num / 10_f64.powf(factor)).round() * 10_f64.powf(factor)
}

#[cfg(test)]
#[test]
fn keep_msf_test() {
	assert_eq!(keep_msf(69., 1), 70.);
	assert_eq!(keep_msf(69.6969, 2), 70.);
	assert_eq!(keep_msf(69.6969, 3), 69.7);
	assert_eq!(keep_msf(69.6969, 4), 69.7);
	assert_eq!(keep_msf(69.6969, 5), 69.697);
	assert_eq!(keep_msf(69.6969, 6), 69.6969);
}
