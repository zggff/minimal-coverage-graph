// Нахождение наименьшего покрытия простого графа.

mod canvas;

use canvas::{Canvas, CanvasData};
use druid::{
    widget::{Flex, Label},
    AppDelegate, AppLauncher, Application, Color, DelegateCtx, Env, Widget, WidgetExt, WindowDesc,
    WindowId,
};

fn main() {
    let main_window = WindowDesc::new(ui_builder())
        .window_size((800.0, 800.0))
        .title("минимальное покрытие простого графа");
    let data = CanvasData::new();
    AppLauncher::with_window(main_window)
        .delegate(Delegate)
        .launch(data)
        .expect("Failed to launch application");
}

fn ui_builder() -> impl Widget<CanvasData> {
    let canvas = Canvas::new();
    let instructions = Flex::column()
        .with_child(Label::new("Правая кнопка мыши - задание вершин"))
        .with_child(Label::new("Синяя вершина - исходящая"))
        .with_child(Label::new("Зеленая вершина - входящая"))
        .with_child(Label::new("Левая кнопка мыши - задание дуг"))
        .with_child(Label::new("Фиолетовые дуги - минимальное покрытие"));

    let label = Label::new(|data: &CanvasData, _env: &Env| {
        if data.coverage_error {
            "Невозможно найти минимальное покрытие"
        } else {
            ""
        }
    })
    .with_line_break_mode(druid::widget::LineBreaking::WordWrap)
    .with_text_color(Color::RED);
    Flex::column()
        .with_child(instructions)
        .with_default_spacer()
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
