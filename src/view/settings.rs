use crate::gui::styles::colors::*;
use super::MyViewF;
use crate::gui::widgets::{FancyHorSlider, Toggle};
use fltk::{enums::*, prelude::*, *};
use fltk_grid::Grid;
use std::sync::atomic::Ordering;

fn fill_grid(grid: &mut Grid, view: &MyViewF) {
    let mut f = frame::Frame::default()
        .with_align(Align::Left | Align::Inside)
        .with_label("Light mode:");
    grid.insert(&mut f, 3..6, 2);
    let mut t = Toggle::default();
    t.set_value(view.light_mode.load(Ordering::Relaxed));
    let light_mode = view.light_mode.clone();
    t.set_callback(move |t| {
        if t.value() {
            app::foreground(50, 50, 50);
            app::background(255, 255, 255);
            light_mode.store(true, Ordering::Relaxed);
        } else {
            app::foreground(255, 255, 255);
            let (r, g, b) = GRAY.to_rgb();
            app::background(r, g, b);
            light_mode.store(false, Ordering::Relaxed);
        }
        app::redraw();
    });
    grid.insert(&mut *t, 3..5, 15);
    let mut f = frame::Frame::default()
        .with_align(Align::Left | Align::Inside)
        .with_label("Sleep duration:");

  
}

pub fn settings(view: &MyViewF) -> Option<Box<dyn FnMut() + Send>> {
    let mut grp = group::Pack::default_fill().center_of_parent();
    grp.set_spacing(0);
    let mut grid = Grid::default_fill();
    grid.set_layout(20, 20);
    // grid.debug(true);
    grp.end();
    fill_grid(&mut grid, view);
    None
}
