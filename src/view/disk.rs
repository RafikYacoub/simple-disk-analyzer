use super::MyViewF;
use crate::{
    gui::styles::colors::*,
    gui::widgets::{Card, Dial},
};
use fltk::{prelude::*, *};
use sysinfo::DiskExt;
use sysinfo::SystemExt;

pub fn disks(view: &MyViewF) -> Option<Box<dyn FnMut() + Send>> {

    let mut sys = view.system.lock();
    
    sys.refresh_disks();
    frame::Frame::new(60, 60, 0, 0, None);
    let mut grp = group::Pack::default()
        .with_size(600, 400)
        .center_of_parent();
    grp.set_spacing(40);
    for disk in sys.disks() {
        let mut hpack = group::Pack::default()
            .with_size(600, 130)
            .with_type(group::PackType::Horizontal);
        hpack.set_spacing(50);
        let t = Card::default()
            .with_size(300, 130)
            .with_label(disk.name().to_str().unwrap());
        t.begin();
        let vpack = group::Pack::default().with_size(600, 100).center_of_parent();
        let mut f = frame::Frame::default()
        .with_size(80, 35)
        .with_label(&format!(
            "{}",
            disk.mount_point().to_str().unwrap(),
        ));
        f.set_label_size(14);
        frame::Frame::default()
            .with_size(80, 35)
            .with_label(&format!(
                "{:?}: {} - Space: {:.02} GiB",
                disk.type_(),
                String::from_utf8(disk.file_system().to_vec()).unwrap(),
                disk.total_space() as f64 / 2_f64.powf(30.)
            ));
        frame::Frame::default()
        .with_size(80, 35)
        .with_label(&format!(
            "Removable: {}", if disk.is_removable() {"Yes"} else {"No"}
        ));
        vpack.end();
        t.end();
        let grp = group::Group::default().with_size(130, 130);
        let mut dial = Dial::default()
            .with_size(100, 100)
            .with_label("Used space %")
            .center_of_parent();
        dial.modifiable(false);
        dial.set_value(
            ((disk.total_space() - disk.available_space()) as f64 * 100.
                / disk.total_space() as f64) as i32,
        );
        dial.set_selection_color(DISK_PURPLE);
        grp.end();
        hpack.end();
    }
    grp.end();
    None
}
