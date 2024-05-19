use std::collections::BTreeSet;

use druid::kurbo::{Ellipse, Line, Rect};
use druid::piet::FontFamily;
use druid::text::{FontDescriptor, TextLayout};
use druid::{theme, RenderContext};
use druid::{Color, Data, Event, Lens, Point, Size, Widget};

use crate::GRAPH_COVERAGE;

static RADIUS: f64 = 15.0;

pub struct Canvas {
    pub x: Vec<Point>,
    pub y: Vec<Point>,
    // pub nodes: Vec<(Point, Point)>,
    pub nodes: BTreeSet<(usize, usize)>,
    pub node: Option<(usize, Point)>,
    pub coverage: Option<Vec<(Point, Point)>>,
}

impl Canvas {
    pub fn new() -> Self {
        Canvas {
            x: Vec::new(),
            y: Vec::new(),
            nodes: BTreeSet::new(),
            node: None,
            coverage: None,
        }
    }
}

#[derive(Clone, Data, Lens, Debug)]
pub struct CanvasData {
    pub coverage_error: bool,
    pub vertice_type: bool,
}

impl CanvasData {
    pub fn new() -> Self {
        CanvasData {
            coverage_error: false,
            vertice_type: false,
        }
    }
}

impl Widget<CanvasData> for Canvas {
    fn paint(
        &mut self,
        ctx: &mut druid::widget::prelude::PaintCtx,
        _data: &CanvasData,
        env: &druid::widget::prelude::Env,
    ) {
        let background = Rect::from_origin_size((0.0, 0.0), ctx.size());
        ctx.fill(background, &env.get(theme::BACKGROUND_LIGHT));

        for &(x, y) in &self.nodes {
            // ctx.stroke(Line::new(x, y), &Color::RED, 2.0);
            ctx.stroke(Line::new(self.x[x], self.y[y]), &Color::RED, 2.0);
        }
        if let Some((x, y)) = self.node {
            ctx.stroke(Line::new(self.x[x], y), &Color::BLACK, 1.0);
        }
        if let Some(coverage) = &self.coverage {
            for &(x, y) in coverage {
                ctx.stroke(Line::new(x, y), &Color::PURPLE, 2.0);
            }
        }

        for (i, p) in self.x.iter().enumerate() {
            let ellipse = Ellipse::new(*p, (RADIUS, RADIUS), 0.0);
            ctx.fill(ellipse, &env.get(theme::BACKGROUND_LIGHT));
            ctx.stroke(ellipse, &Color::GREEN, 2.0);
            let mut layout = TextLayout::<String>::from_text(format!("{i}"));
            layout.set_font(FontDescriptor::new(FontFamily::SERIF).with_size(RADIUS));
            layout.set_text_color(Color::GREEN);
            layout.rebuild_if_needed(ctx.text(), env);
            let s = layout.size();
            layout.draw(ctx, (p.x - s.width / 2.0, p.y - s.height / 2.0));
        }
        for (i, p) in self.y.iter().enumerate() {
            let ellipse = Ellipse::new(*p, (RADIUS, RADIUS), 0.0);
            ctx.fill(ellipse, &env.get(theme::BACKGROUND_LIGHT));
            ctx.stroke(ellipse, &Color::BLUE, 2.0);
            let mut layout = TextLayout::<String>::from_text(format!("{i}"));
            layout.set_font(FontDescriptor::new(FontFamily::SERIF).with_size(10.0));
            layout.set_text_color(Color::BLUE);
            layout.rebuild_if_needed(ctx.text(), env);
            let s = layout.size();
            layout.draw(ctx, (p.x - s.width / 2.0, p.y - s.height / 2.0));
        }
    }

    fn layout(
        &mut self,
        ctx: &mut druid::widget::prelude::LayoutCtx,
        bc: &druid::widget::prelude::BoxConstraints,
        _data: &CanvasData,
        _env: &druid::widget::prelude::Env,
    ) -> druid::widget::prelude::Size {
        let default_size = Size::new(
            ctx.window().get_size().width,
            ctx.window().get_size().height,
        );

        bc.constrain(default_size)
    }

    fn event(
        &mut self,
        ctx: &mut druid::widget::prelude::EventCtx,
        event: &druid::widget::prelude::Event,
        _data: &mut CanvasData,
        _env: &druid::widget::prelude::Env,
    ) {
        match event {
            Event::MouseDown(e) => match e.button {
                druid::MouseButton::Right => {
                    'r: {
                        for (i, &p) in self.x.iter().enumerate() {
                            if p.distance(e.pos) <= RADIUS {
                                self.nodes = self
                                    .nodes
                                    .iter()
                                    .copied()
                                    .filter(|&(x, _)| x != i)
                                    .map(|(x, y)| if x < i { (x, y) } else { (x - 1, y) })
                                    .collect();
                                self.y.push(self.x.remove(i));
                                break 'r;
                            }
                        }
                        for (i, &p) in self.y.iter().enumerate() {
                            if p.distance(e.pos) <= RADIUS {
                                self.nodes = self
                                    .nodes
                                    .iter()
                                    .copied()
                                    .filter(|&(_, y)| y != i)
                                    .map(|(x, y)| if y < i { (x, y) } else { (x, y - 1) })
                                    .collect();
                                self.y.remove(i);
                                break 'r;
                            }
                        }
                        self.x.push(e.pos);
                    }
                    ctx.request_paint();
                    ctx.request_layout();
                }
                druid::MouseButton::Left => {
                    for (i, &p) in self.x.iter().enumerate() {
                        if e.pos.distance(p) <= RADIUS {
                            self.node = Some((i, p));
                            break;
                        }
                    }
                }
                _ => {}
            },
            Event::MouseMove(e) => {
                self.node = self.node.map(|(x, _)| (x, e.pos));
                if self.node.is_some() {
                    ctx.request_paint();
                    ctx.request_layout();
                }
            }
            Event::MouseUp(e) => match e.button {
                druid::MouseButton::Left => {
                    for (j, &p) in self.y.iter().enumerate() {
                        if e.pos.distance(p) <= RADIUS {
                            let (i, _) = self.node.unwrap_or_default();
                            if self.nodes.contains(&(i, j)) {
                                self.nodes.remove(&(i, j));
                            } else {
                                self.nodes.insert((i, j));
                            }
                            // if self.nodes.contains(&node) {
                            //     self.nodes =
                            //         self.nodes.iter().copied().filter(|&n| n != node).collect();
                            // } else {
                            //     self.nodes.push(node);
                            // }
                            break;
                        }
                    }
                    self.node = None;
                    ctx.request_paint();
                    ctx.request_layout();
                }
                _ => {}
            },
            Event::Command(cmd) => {
                if cmd.is(GRAPH_COVERAGE) {
                    todo!("not done");
                }
            }
            _ => {}
        }
    }

    fn update(
        &mut self,
        _ctx: &mut druid::widget::prelude::UpdateCtx,
        _old_data: &CanvasData,
        _data: &CanvasData,
        _env: &druid::widget::prelude::Env,
    ) {
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut druid::widget::prelude::LifeCycleCtx,
        _event: &druid::widget::prelude::LifeCycle,
        _data: &CanvasData,
        _env: &druid::widget::prelude::Env,
    ) {
    }
}
