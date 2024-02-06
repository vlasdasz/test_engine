use test_engine::{
    gm::Color,
    ui,
    ui::{refs::Weak, Container, SubView, ViewData, ViewSetup},
    view,
};

#[view]
pub struct WGPUTestView {
    tl: SubView<Container>,
    tr: SubView<Container>,
    bl: SubView<Container>,
    br: SubView<Container>,
}

impl ViewSetup for WGPUTestView {
    fn setup(mut self: Weak<Self>) {
        self.set_color(Color::LIGHTER_GRAY);

        self.tl.set_color(Color::RED).place().size(100, 100).tl(10);
        self.tr.set_color(Color::GREEN).place().size(100, 100).tr(10);
        self.bl.set_color(Color::BLUE).place().size(100, 100).bl(10);
        self.br.set_color(Color::ORANGE).place().size(100, 100).br(10);
    }
}