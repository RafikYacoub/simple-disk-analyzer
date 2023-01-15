use super::MyViewF;
use fltk::{enums::*, prelude::*, *};

const INFO: &str = r#"
Diski is a lightweight disk analyzer built in rust and fltk,
the design of the app is built upon sysinfo-gui
"#;


pub fn info(_view: &MyViewF) -> Option<Box<dyn FnMut() + Send>> {
    let mut grp = group::Pack::default()
    .with_size(600, 400)
    .center_of_parent();
    grp.set_spacing(40);
    let mut frame = misc::HelpView::default()
        .with_size(500, 300)
        .center_of_parent();
    frame.set_frame(FrameType::FlatBox);
    frame.set_color(frame.parent().unwrap().color());
    frame.set_value(INFO);
    frame.set_text_size(16);
    frame.set_text_font(Font::Helvetica);
    grp.end();
    None
}

