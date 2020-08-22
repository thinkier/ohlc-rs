use serde::export::PhantomData;

use model::*;

#[derive(Clone, Debug)]
pub struct TestLine<C>(pub PhantomData<C>);

impl<C: Candle> RendererExtension for TestLine<C> {
    type Candle = C;

    fn apply(&self, buffer: &mut ChartBuffer, _data: &[C]) {
        buffer.line((0, 0), (1309, 649), 0xFFFF00FF);
        buffer.line((0, 649), (1309, 0), 0xFFFF00FF);
        buffer.line((0, 0), (0, 649), 0xFFFF00FF);
        buffer.line((0, 649), (1309, 649), 0xFFFF00FF);
        buffer.line((1309, 649), (1309, 0), 0xFFFF00FF);
        buffer.line((0, 0), (1309, 0), 0xFFFF00FF);
    }

    fn lore_colour(&self) -> Option<u32> {
        None
    }

    fn name(&self) -> String {
        "TEST_Line()".to_string()
    }
}

impl<C> PartialEq for TestLine<C> {
    fn eq(&self, _: &TestLine<C>) -> bool {
        true
    }
}