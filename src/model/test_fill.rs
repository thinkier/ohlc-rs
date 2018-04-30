use api::RendererExtension;
use data::OHLC;
use model::ChartBuffer;

#[derive(Debug)]
pub struct TestFill;

impl RendererExtension for TestFill {
	fn apply(&self, buffer: &mut ChartBuffer, _data: &[OHLC]) {
		buffer.rect(0, 0, 200, 200, 0xFFFF00FF);
	}

	fn name(&self) -> String {
		"NoExtension()".to_string()
	}
}

impl PartialEq for TestFill {
	fn eq(&self, _: &TestFill) -> bool {
		true
	}
}