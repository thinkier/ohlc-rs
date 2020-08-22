use model::*;

#[derive(Clone, Debug)]
pub struct NoExtension;

impl RendererExtension for NoExtension {
    fn apply(&self, _buffer: &mut ChartBuffer, _data: &[OHLC]) {}

    fn lore_colour(&self) -> Option<u32> {
        None
    }

    fn name(&self) -> String {
        "NoExtension()".to_string()
    }
}

impl PartialEq for NoExtension {
    fn eq(&self, _: &NoExtension) -> bool {
        true
    }
}