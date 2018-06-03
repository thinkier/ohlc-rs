use model::*;

#[derive(Clone, Debug)]
pub struct TestFill {
	pub colour: u32
}

impl RendererExtension for TestFill {
	fn apply(&self, buffer: &mut ChartBuffer, _data: &[OHLC]) {
		buffer.rect(0, 0, 200, 200, self.colour);
	}

	fn lore_colour(&self) -> Option<u32> {
		None
	}

	fn name(&self) -> String {
		"TEST_Fill()".to_string()
	}
}

impl PartialEq for TestFill {
	fn eq(&self, _: &TestFill) -> bool {
		true
	}
}