// Нахождение наименьшего покрытия простого графа.

mod canvas;

use canvas::{Canvas, CanvasData};
use druid::{
    widget::{Button, Flex, Label, List},
    AppDelegate, AppLauncher, Application, Color, DelegateCtx, Env, Widget, WidgetExt, WindowDesc,
    WindowId,
};

fn main() {
    let main_window = WindowDesc::new(ui_builder())
        .window_size((1600.0, 800.0))
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

    let table_y = List::new(|| {
        Label::new(|data: &usize, _env: &Env| format!("Y{data}"))
            .center()
            .fix_size(30.0, 30.0)
            .border(Color::TRANSPARENT, 1.0)
    })
    .horizontal()
    .lens(CanvasData::y);

    let table_x = List::new(|| {
        Label::new(|data: &usize, _env: &Env| format!("X{data}"))
            .center()
            .fix_size(30.0, 30.0)
            .border(Color::TRANSPARENT, 1.0)
    })
    .lens(CanvasData::x);

    let table_data = List::new(|| {
        List::new(|| {
            Button::new(|data: &u8, _env: &Env| format!("{data}"))
                .on_click(|_, data: &mut u8, _| *data = if *data == 0 { 1 } else { 0 })
                .fix_size(30.0, 30.0)
                .border(Color::WHITE, 1.0)
        })
        .horizontal()
    })
    .lens(CanvasData::mat);

    let empty = Label::new("").fix_size(30.0, 30.0);
    let table_y = Flex::row().with_child(empty).with_child(table_y);

    let table = Flex::row().with_child(table_x).with_child(table_data);
    let table = Flex::column()
        .with_child(table_y)
        .with_child(table)
        .border(Color::WHITE, 2.0);

    let workspace = Flex::row()
        .with_child(canvas)
        .with_default_spacer()
        .with_child(table);
    Flex::column()
        .with_child(instructions)
        .with_default_spacer()
        .with_child(label)
        .with_default_spacer()
        .with_child(workspace)
        .padding(5.0)
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
