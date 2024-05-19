// Нахождение наименьшего покрытия простого графа.

mod canvas;

use canvas::{Canvas, CanvasData};
use druid::{
    widget::{Button, Flex, Label},
    AppLauncher, Env, Selector, Widget, WidgetExt, WindowDesc,
};

const GRAPH_COVERAGE: Selector = Selector::new("graph_coverage.compute_graph_coverage");

fn main() {
    let main_window = WindowDesc::new(ui_builder());
    let data = CanvasData::new();
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(data)
        .expect("Failed to launch application");
}

fn ui_builder() -> impl Widget<CanvasData> {
    let canvas = Canvas::new();
    let button =
        Button::new("Zggff").on_click(|ctx, _data, _env| ctx.submit_command(GRAPH_COVERAGE));
    let label = Label::new(|data: &CanvasData, _env: &Env| format!("{}", data.vertice_type));
    let controls = Flex::column()
        .with_child(button)
        .with_default_spacer()
        .with_child(label)
        .padding(10.0);
    Flex::row().with_child(controls).with_child(canvas)
}
