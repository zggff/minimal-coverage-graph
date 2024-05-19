// Нахождение наименьшего покрытия простого графа.

mod canvas;

use canvas::{Canvas, CanvasData};
use druid::{
    widget::{Button, Flex, Label},
    AppLauncher, Env, Key, Selector, Widget, WidgetExt, WindowDesc,
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
    let button = Button::new("Найти минимальное покрытие")
        .on_click(|ctx, _data, _env| ctx.submit_command(GRAPH_COVERAGE));
    let label = Label::new(|data: &CanvasData, _env: &Env| {
        if data.coverage_error {
            "Невозможно найти минимальное покрытие"
        } else {
            ""
        }
    })
    .with_line_break_mode(druid::widget::LineBreaking::WordWrap);
    Flex::column()
        .with_child(button)
        .with_default_spacer()
        .with_child(label)
        .with_default_spacer()
        .with_child(canvas)
        .padding(10.0)
}
