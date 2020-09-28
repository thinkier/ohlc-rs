use std::marker::PhantomData;

use model::*;

#[derive(Clone, Debug)]
pub struct NoExtension<C>(PhantomData<C>);

impl<C: Candle> RendererExtension for NoExtension<C> {
	type Candle = C;

	fn apply(&self, _buffer: &mut ChartBuffer, _data: &[C]) {}

	fn lore_colour(&self) -> Option<u32> {
		None
	}

	fn name(&self) -> String {
		"NoExtension()".to_string()
	}
}

impl<C> PartialEq for NoExtension<C> {
	fn eq(&self, _: &NoExtension<C>) -> bool {
		true
	}
}