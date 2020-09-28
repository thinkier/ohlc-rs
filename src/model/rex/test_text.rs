use serde::export::PhantomData;

use model::*;

#[derive(Clone, Debug)]
pub struct TestText<C>(pub PhantomData<C>);

impl<C: Candle> RendererExtension for TestText<C> {
	type Candle = C;

	fn apply(&self, buffer: &mut ChartBuffer, _data: &[C]) {
		buffer.text((0, 0), "DANKMEME", 0xFFFF00FF);
		buffer.text((0, 60), "DANKMEME", 0xFFFF007F);
	}

	fn lore_colour(&self) -> Option<u32> {
		None
	}

	fn name(&self) -> String {
		"TEST_Text()".to_string()
	}
}

impl<C> PartialEq for TestText<C> {
	fn eq(&self, _: &TestText<C>) -> bool {
		true
	}
}