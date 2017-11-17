/// Options struct for axis.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AxisOptions {
	/// Title of the axis
	pub(crate) title: String,
	/// RGBA(8) Colour for the title of the axis
	pub(crate) title_colour: u32,
	/// RGBA(8) Colour used for lines drawn upon the graph
	pub(crate) line_colour: u32,
	/// Frequency of lines drawn
	pub(crate) line_frequency: f64,
	/// RGBA(8) Colour used for the labels
	pub(crate) label_colour: u32,
	/// Frequency of labelling
	///
	/// The frequency is based on seconds for time, using data from [`OHLCRenderOptions.time_units`]
	///
	/// Setting this to 0 will disable labelling of values.
	pub(crate) label_frequency: f64,
}

impl AxisOptions {
	pub fn new() -> AxisOptions {
		AxisOptions {
			title: String::new(),
			// 100% Opaque black
			title_colour: 0x000000FF,
			line_colour: 0x999999FF,
			line_frequency: 0.0,
			label_colour: 0x555555FF,
			label_frequency: 0.0,
		}
	}

	pub fn title(mut self, title: &str, colour: u32) -> Self {
		self.title = title.to_string();
		self.title_colour = colour;

		self
	}

	pub fn line(mut self, colour: u32, frequency: f64) -> Self {
		self.line_colour = colour;
		self.line_frequency = frequency;

		self
	}

	pub fn label(mut self, colour: u32, frequency: f64) -> Self {
		self.label_colour = colour;
		self.label_frequency = frequency;

		self
	}
}
