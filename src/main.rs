// Нахождение наименьшего покрытия простого графа.

mod canvas;

use canvas::{Canvas, CanvasData};
use druid::{
    widget::{Flex, Label},
    AppDelegate, AppLauncher, Application, DelegateCtx, Env, Selector, Widget, WidgetExt,
    WindowDesc, WindowId,
};

const GRAPH_COVERAGE: Selector = Selector::new("graph_coverage.compute_graph_coverage");

fn main() {
    let main_window = WindowDesc::new(ui_builder());
    let data = CanvasData::new();
    AppLauncher::with_window(main_window)
        .delegate(Delegate)
        .launch(data)
        .expect("Failed to launch application");
}

fn ui_builder() -> impl Widget<CanvasData> {
    let canvas = Canvas::new();
    let label = Label::new(|data: &CanvasData, _env: &Env| {
        if data.coverage_error {
            "Невозможно найти минимальное покрытие"
        } else {
            ""
        }
    })
    .with_line_break_mode(druid::widget::LineBreaking::WordWrap);
    Flex::column()
        .with_child(label)
        .with_default_spacer()
        .with_child(canvas)
        .padding(10.0)
}

// correctly handle closing the window on macos
struct Delegate;

impl AppDelegate<CanvasData> for Delegate {
    fn window_removed(
        &mut self,
        _id: WindowId,
        _data: &mut CanvasData,
        _env: &Env,
        _ctx: &mut DelegateCtx,
    ) {
        Application::global().quit();
    }
}
