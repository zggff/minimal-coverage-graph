use std::collections::BTreeSet;

use druid::kurbo::{Ellipse, Line, Rect};
use druid::piet::FontFamily;
use druid::text::{FontDescriptor, TextLayout};
use druid::{theme, RenderContext};
use druid::{Color, Data, Event, Lens, Point, Size, Widget};

const RADIUS: f64 = 15.0;

#[derive(Clone, Data, Lens, Debug)]
pub struct CanvasData {
    pub coverage_error: bool,
}

impl CanvasData {
    pub fn new() -> Self {
        CanvasData {
            coverage_error: false,
        }
    }
}

pub struct Canvas {
    pub x: Vec<Point>,
    pub y: Vec<Point>,
    pub node_origin_in_x: bool,
    pub node: Option<(usize, Point)>,
    pub nodes: BTreeSet<(usize, usize)>,
    pub coverage: BTreeSet<(usize, usize)>,
}

impl Canvas {
    pub fn new() -> Self {
        Canvas {
            x: Vec::new(),
            y: Vec::new(),
            nodes: BTreeSet::new(),
            node_origin_in_x: true,
            node: None,
            coverage: BTreeSet::new(),
        }
    }
    fn compute_coverage(&mut self, data: &mut CanvasData) {
        let mut mat = vec![vec![0_u8; self.y.len()]; self.x.len()];
        let mut out = vec![0; self.x.len()]; //  F
        let mut inp = vec![0; self.y.len()]; //  G

        // convert into matrix
        for &(i, j) in &self.nodes {
            mat[i][j] = 1;
            out[i] += 1;
            inp[j] += 1;
        }

        // check if it is possible to find coverage
        if out.iter().any(|&v| v == 0) || inp.iter().any(|&v| v == 0) {
            data.coverage_error = true;
            return;
        }

        // brute force first part
        for i in 0..self.x.len() {
            for j in 0..self.y.len() {
                if inp[j] > 1 && out[i] > 1 && mat[i][j] == 1 {
                    mat[i][j] = 2;
                    out[i] -= 1;
                    inp[j] -= 1;
                }
            }
        }

        // fix any errors
        let mut i0 = 0;
        while i0 < self.x.len() {
            if out[i0] == 1 {
                i0 += 1;
                continue;
            }
            let mut found = false;
            'top: for j0 in 0..self.y.len() {
                if mat[i0][j0] != 1 {
                    continue;
                }
                for i1 in 0..self.x.len() {
                    if mat[i1][j0] != 2 {
                        continue;
                    }
                    for j1 in 0..self.y.len() {
                        if mat[i1][j1] != 1 {
                            continue;
                        }
                        if inp[j1] == 1 {
                            continue;
                        }
                        out[i0] -= 1;
                        inp[j1] -= 1;
                        mat[i0][j0] = 2;
                        mat[i1][j0] = 1;
                        mat[i1][j1] = 2;
                        found = true;
                        break 'top;
                    }
                }
            }
            if !found {
                i0 += 1;
            }
        }

        // convert back into correct form
        for i in 0..self.x.len() {
            for j in 0..self.y.len() {
                if mat[i][j] == 1 {
                    self.coverage.insert((i, j));
                }
            }
        }
        data.coverage_error = false;
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
            let color = Color::rgb(1.0, 0.5, 0.5);
            ctx.stroke(Line::new(self.x[x], self.y[y]), &color, 2.0);
        }
        if let Some((x, y)) = self.node {
            let x = if self.node_origin_in_x {
                self.x[x]
            } else {
                self.y[x]
            };
            ctx.stroke(Line::new(x, y), &Color::BLACK, 1.0);
        }
        for &(x, y) in &self.coverage {
            ctx.stroke(Line::new(self.x[x], self.y[y]), &Color::PURPLE, 2.0);
        }

        for (i, p) in self.x.iter().enumerate() {
            let ellipse = Ellipse::new(*p, (RADIUS, RADIUS), 0.0);
            let color = Color::rgb(0.5, 0.5, 1.0);

            ctx.fill(ellipse, &env.get(theme::BACKGROUND_LIGHT));
            ctx.stroke(ellipse, &color, 2.0);
            let mut layout = TextLayout::<String>::from_text(format!("{}", i + 1));
            layout.set_font(FontDescriptor::new(FontFamily::SERIF).with_size(RADIUS));
            layout.set_text_color(color);
            layout.rebuild_if_needed(ctx.text(), env);
            let s = layout.size();
            layout.draw(ctx, (p.x - s.width / 2.0, p.y - s.height / 2.0));
        }
        for (i, p) in self.y.iter().enumerate() {
            let ellipse = Ellipse::new(*p, (RADIUS, RADIUS), 0.0);
            let color = Color::rgb(0.5, 1.0, 0.5);

            ctx.fill(ellipse, &env.get(theme::BACKGROUND_LIGHT));
            ctx.stroke(ellipse, &color, 2.0);
            let mut layout = TextLayout::<String>::from_text(format!("{}", i + 1));
            layout.set_font(FontDescriptor::new(FontFamily::SERIF).with_size(RADIUS));
            layout.set_text_color(color);
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
        data: &mut CanvasData,
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
                    self.coverage.clear();
                    self.compute_coverage(data);
                    ctx.request_paint();
                    ctx.request_layout();
                }
                druid::MouseButton::Left => {
                    for (i, &p) in self.x.iter().enumerate() {
                        if e.pos.distance(p) <= RADIUS {
                            self.node = Some((i, p));
                            self.node_origin_in_x = true;
                            break;
                        }
                    }
                    for (i, &p) in self.y.iter().enumerate() {
                        if e.pos.distance(p) <= RADIUS {
                            self.node = Some((i, p));
                            self.node_origin_in_x = false;
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
                    let iter = if self.node_origin_in_x {
                        self.y.iter()
                    } else {
                        self.x.iter()
                    };
                    for (j, &p) in iter.enumerate() {
                        if e.pos.distance(p) <= RADIUS {
                            let (i, _) = self.node.unwrap_or_default();
                            let node = if self.node_origin_in_x {
                                (i, j)
                            } else {
                                (j, i)
                            };
                            if self.nodes.contains(&node) {
                                self.nodes.remove(&node);
                            } else {
                                self.nodes.insert(node);
                            }
                            self.coverage.clear();
                            self.compute_coverage(data);
                            break;
                        }
                    }
                    self.node = None;
                    ctx.request_paint();
                    ctx.request_layout();
                }
                _ => {}
            },
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
