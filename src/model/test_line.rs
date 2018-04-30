use api::RendererExtension;
use data::OHLC;
use model::ChartBuffer;

#[derive(Debug)]
pub struct TestLine;

impl RendererExtension for TestLine {
	fn apply(&self, buffer: &mut ChartBuffer, _data: &[OHLC]) {
		buffer.line((0, 0), (1310, 650),  0xFFFF00FF);
		buffer.line((0, 650), (1310, 0),  0xFFFF00FF);
	}

	fn name(&self) -> String {
		"NoExtension()".to_string()
	}
}

impl PartialEq for TestLine {
	fn eq(&self, _: &TestLine) -> bool {
		true
	}
}