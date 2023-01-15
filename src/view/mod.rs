pub mod cpu;
pub mod disk;
pub mod general;
pub mod tps;
pub mod procs;
pub mod settings;
pub mod info;
pub mod lib;

use crate::gui::{message::Message, styles::colors::GRAY, View};
use fltk::app;
use parking_lot::Mutex;
use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    Arc,
};
use sysinfo::{System, SystemExt};

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq)]
enum SortFiles
{
    size,
    revSize,
    acc,
    revAcc,
    modif,
    revModif,
    cr,
    revCr,
    typ,
    revTyp,
    dir,
    revDir,
}


pub struct MyViewF {
    system: Arc<Mutex<System>>,
    sleep: Arc<AtomicU64>,
    light_mode: Arc<AtomicBool>,
    ordering: Arc<Mutex<SortFiles>>,
}

impl Default for MyViewF {
    fn default() -> Self {
        #[cfg(feature = "dark-light")]
        let mode = dark_light::detect() == dark_light::Mode::Light;
        #[cfg(not(feature = "dark-light"))]
        let mode = false;

        if mode {
            app::foreground(50, 50, 50);
            app::background(255, 255, 255);
        } else {
            app::foreground(255, 255, 255);
            let (r, g, b) = GRAY.to_rgb();
            app::background(r, g, b);
        }

        let mut sys = System::new_all();
        sys.refresh_all();

        let system = Arc::new(Mutex::new(sys));

        Self {
            system,
            sleep: Arc::new(AtomicU64::from(300)),
            light_mode: Arc::new(AtomicBool::from(mode)),
            ordering: Arc::new(Mutex::new(SortFiles::size)),
        }
    }
}


impl View for MyViewF {
    fn view(&self, msg: Message) -> Option<Box<dyn FnMut() + Send>> {
        match msg {
            Message::General => self.general(),
            Message::Disks => self.disks(),
            Message::tps => self.types(),
            Message::files => self.files(),
            Message::Settings => self.settings(),
            Message::Info => self.info(),
        }
    }
    fn sleep_duration(&self) -> u64 {
        self.sleep.load(Ordering::Relaxed)
    }
    fn light_mode(&self) -> bool {
        self.light_mode.load(Ordering::Relaxed)
    }
}



impl MyViewF {
    pub fn files(&self) -> Option<Box<dyn FnMut() + Send>> {
        procs::files(self)
    }

    pub fn general(&self) -> Option<Box<dyn FnMut() + Send>> {
        general::general(self)
    }
    
    pub fn types(&self) -> Option<Box<dyn FnMut() + Send>> {
        tps::Types(self)
    }

    pub fn settings(&self) -> Option<Box<dyn FnMut() + Send>> {
        settings::settings(self)
    }
    
    pub fn disks(&self) -> Option<Box<dyn FnMut() + Send>> {
        disk::disks(self)
    }

    pub fn info(&self) -> Option<Box<dyn FnMut() + Send>> {
        info::info(self)
    }
}

/* 
impl MyView {
    pub fn general(&self) -> Option<Box<dyn FnMut() + Send>> {
        general::general(self)
    }
    
    pub fn memory(&self) -> Option<Box<dyn FnMut() + Send>> {
        mem::memory(self)
    }
    pub fn settings(&self) -> Option<Box<dyn FnMut() + Send>> {
        settings::settings(self)
    }
    
    pub fn cpu(&self) -> Option<Box<dyn FnMut() + Send>> {
        cpu::proc(self)
    }
    
    pub fn disks(&self) -> Option<Box<dyn FnMut() + Send>> {
        disk::disks(self)
    }
    //pub fn procs(&self) -> Option<Box<dyn FnMut() + Send>> {
    //    procs::procs(self)
    //}
    pub fn info(&self) -> Option<Box<dyn FnMut() + Send>> {
        info::info(self)
    }
}
*/
