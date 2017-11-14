/// Options struct for axis.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AxisOptions {
	/// Title of the axis
	title: String,
	/// RGBA(8) Colour for the title of the axis
	title_colour: u32,
	/// RGBA(8) Colour used for lines drawn upon the graph
	line_colour: u32,
	/// Frequency of lines drawn
	line_frequency: u32,
	/// RGBA(8) Colour used for the labels
	label_colour: u32,
	/// Frequency of labelling
	///
	/// The frequency is based on seconds for time, using data from [`OHLCRenderOptions.time_units`]
	///
	/// Setting this to 0 will disable labelling of values.
	label_frequency: u64,
}

impl AxisOptions {
	pub fn new() -> AxisOptions {
		AxisOptions {
			title: String::new(),
			// 100% Opaque black
			title_colour: 0xFF000000,
			line_colour: 0x7F000000,
			line_frequency: 0,
			label_colour: 0xAF000000,
			label_frequency: 0,
		}
	}

	pub fn title(mut self, title: &str) -> Self {
		self.title = title.to_string();

		self
	}

	pub fn text_colour(mut self, colour: u32) -> Self {
		self.text_colour = colour;

		self
	}

	pub fn label_frequency(mut self, label_frequency: u64) -> Self {
		self.label_frequency = label_frequency;

		self
	}
}