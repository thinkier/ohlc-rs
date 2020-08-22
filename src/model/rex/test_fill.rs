use serde::export::PhantomData;

use model::*;

#[derive(Clone, Debug)]
pub struct TestFill<C> {
    pub _c: PhantomData<C>,
    pub colour: u32,
}

impl<C: Candle> RendererExtension for TestFill<C> {
    type Candle = C;

    fn apply(&self, buffer: &mut ChartBuffer, _data: &[C]) {
        buffer.rect(0, 0, 200, 200, self.colour);
    }

    fn lore_colour(&self) -> Option<u32> {
        None
    }

    fn name(&self) -> String {
        "TEST_Fill()".to_string()
    }
}

impl<C> PartialEq for TestFill<C> {
    fn eq(&self, _: &TestFill<C>) -> bool {
        true
    }
}