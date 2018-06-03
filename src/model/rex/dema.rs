use model::*;
use model::buffer::ChartBuffer;
use model::rex::ema::*;

#[derive(Clone, Debug)]
pub struct DEMA {
	inner: EMA,
}

impl DEMA {
	pub fn new(ema: EMA) -> DEMA {
		DEMA { inner: ema }
	}
}

impl RendererExtension for DEMA {
	fn apply(&self, buffer: &mut ChartBuffer, data: &[OHLC]) {
		let tf = buffer.timeframe;
		let len = data.len();
		let dema = {
			let ema_buf = ema(&self.inner, &median_list(data));
			let mut dema_buf = ema_buf.clone();
			multply_all(&mut dema_buf, 2.);

			subtract(&mut dema_buf, &ema(&self.inner, &ema_buf));

			dema_buf
		};

		for p in self.inner.periods + 1..len {
			let p1 = buffer.data_to_coords(dema[p - 1], (tf as f64 * ((p - 1) as f64 / len as f64)) as i64);
			let p2 = buffer.data_to_coords(dema[p], (tf as f64 * (p as f64 / len as f64)) as i64);

			buffer.line(p1, p2, self.inner.colour);
		}
	}

	fn name(&self) -> String {
		format!("DEMA({}, sf={})", self.inner.periods, self.inner.smoothing_factor)
	}
}

pub fn multply_all(buf: &mut [f64], factor: f64) {
	for i in 0..buf.len() {
		buf[i] *= factor;
	}
}

pub fn subtract(buf: &mut [f64], other: &[f64]) {
	let len = buf.len();
	if len != other.len() {
		return;
	}

	for i in 0..len {
		buf[i] -= other[i];
	}
}