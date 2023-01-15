
use std::{fs, string};
use std::path::Path;

extern crate walkdir;
use walkdir::WalkDir;


use chrono::prelude::{DateTime, Utc};

use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};
use sysinfo::DiskExt;

/* 
// handling the file structure start ----------------------------------------------------------------------------------------
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct fileNode
{
    // size in kb
    size: u32,
    //last accessed
    acc: String,
    modif: String,
    cre: String,
    // parent dir
    //parent: u32,
    // type of file
    tp: String,
    dir: String,
    // last modified, accessed, created 
}

impl fileNode {
    pub fn new(size: u32, acc: String, modif: String, cre: String, tp: String, dir: String) -> Self {
        fileNode { size, acc, modif, cre, tp, dir }
    }
}
*/


struct folderNode
{

}





// handling the file structure end -------------------------------------------------------------------------------------------

// utilities start ----------------------------------------------------------------------------------------------------------

fn iso8601(st: &std::time::SystemTime) -> String {
    let dt: DateTime<Utc> = st.clone().into();
    //https://docs.rs/chrono/0.4.23/chrono/format/strftime/index.html
    format!("{}", dt.format("%v"))
    // formats like "2001-07-08T00:34:60.026490+09:30"
}

fn unix_time(st: &std::time::SystemTime) -> String {
    let dt: DateTime<Utc> = st.clone().into();
    //https://docs.rs/chrono/0.4.23/chrono/format/strftime/index.html
    format!("{}", dt.format("%s"))
    // formats like "2001-07-08T00:34:60.026490+09:30"
}

fn check_ext_err(dir: &str) -> bool
{
    let exten =  Path::new(dir).extension();
    if exten == None || exten.unwrap() == "ini" || exten.unwrap() == "so"
    {
        return true;
    }
    else 
    {
        return false;
    }
}

// utilities end ----------------------------------------------------------------------------------------------------------


// dir display start ----------------------------------------------------------------------------------------------------------
pub fn file_in_dir(dir: &str)
{
    for file in fs::read_dir(dir).unwrap()
    {
        println!("{}", file.unwrap().path().display());
    }
}

pub fn recurs_file_in_dir(dir: &str)
{
    for file in WalkDir::new(dir).into_iter().filter_map(|file| file.ok())
    {
        println!("{}", file.path().display());
    }
}

// dir display end ----------------------------------------------------------------------------------------------------------


pub fn delete_file(dir: &str)
{
    // note if file deletion does not happen the program panics
    // handle this in a better way
    fs::remove_file(dir).expect("File deletion failed!");
}


pub fn delete_dir_content(dir: &str)
{
    // note if file deletion does not happen the program panics
    // handle this in a better way
    fs::remove_dir_all(dir).expect("Directory deletion faile!");
}

// get metadata formatted start ----------------------------------------------------------------------------------------------------------

// development function
/* 
pub fn get_metadata(dir: &str)
{
    let res = fs::metadata(dir);

    let md = res.unwrap();

    // differnt kinds of metadata
    println!("{:?}", md.file_type());

    println!("{:?}", md.is_dir());
    // what's in the documentation: assert!(!metadata.is_dir());

    println!("{:?}", md.is_file());
    // assert!(metadata.is_file());

    // size of file in bytes
    println!("{:?}", md.len());
    //let size_in_MB md.len()/1000000


    println!("{:?}", md.permissions());
    // assert!(!metadata.permissions().readonly());

    //println!("{:?}", md.modified());
    if let Ok(timeMod) = md.modified() {
        iso8601(&timeMod);
        } else {
            println!("Not supported on this platform");
        }
    

    //println!("{:?}", md.accessed());
    if let Ok(timeAcc) = md.accessed() {
        println!("{:?}", timeAcc);
        } else {
            println!("Not supported on this platform");
        }
    

    //println!("{:?}", md.created());
    if let Ok(timeCr) = md.created() {
        println!("{:?}", timeCr);
        } else {
            println!("Not supported on this platform");
        }
    



}
*/ 


pub fn get_size(dir: &str)
{
    let res = fs::metadata(dir);

    let md = res.unwrap();

    let sz = md.len();
    let sz_b = sz as f32;
    let sz_k = sz_b / 1024.0;
    let sz_m = sz_k / 1024.0;
    let sz_g = sz_m / 1024.0;

    println!("{:?}", sz_b);
    if sz_g > 0.1
    {
        println!("size in gb: {:?} gb", sz_g);
    }
    else if sz_m > 0.1
    {
        println!("size in mb: {:?} mb", sz_m);
    }
    else if sz_k > 0.1
    {
        println!("size in kb: {:?} kb", sz_k);
    }
    
    
}



pub fn get_file_type(dir: &str)
{
    let res = fs::metadata(dir);
    let md = res.unwrap();

    if md.file_type().is_symlink()
    {
        println!("this is a shortcut");
    }
    else if md.file_type().is_dir()
    {
        println!("this is a directory");
    }
    else if md.file_type().is_file()
    {
        let exten =  Path::new(dir).extension().unwrap();
        println!("{:?}", exten);
    }
    else
    {
        println!("unexpected file type");
    }
    
}

pub fn get_modified(dir: &str)
{
    let res = fs::metadata(dir);
    let md = res.unwrap();

    if let Ok(time_mod) = md.modified()
    {
    println!("{:?}", iso8601(&time_mod));
    } 
    else
    {
        println!("Not supported on this platform");
    }
}

pub fn get_accessed(dir: &str)
{
    let res = fs::metadata(dir);
    let md = res.unwrap();

    if let Ok(time_acc) = md.accessed() 
    {
    println!("{:?}", iso8601(&time_acc));
    } 
    else
    {
        println!("Not supported on this platform");
    }
}

pub fn get_created(dir: &str)
{
    let res = fs::metadata(dir);
    let md = res.unwrap();

    if let Ok(time_cr) = md.created() 
    {
    println!("{:?}", iso8601(&time_cr));
    } 
    else
    {
        println!("Not supported on this platform");
    }
}



pub fn get_file_type_all(dir: &str)
{
    for file in WalkDir::new(dir).into_iter().filter_map(|file| file.ok())
    {
        println!("{:?}", file.path().to_str().unwrap());
        get_file_type( file.path().to_str().unwrap());
    }
}

// get metadata formatted end ----------------------------------------------------------------------------------------------------------

// get metadata raw start ----------------------------------------------------------------------------------------------------------



pub fn get_size_r(dir: &str) -> f32
{

    if check_ext_err(dir)
    {
        return 0.0;
    }

    let res = fs::metadata(dir);
    let md = res.unwrap();

    let sz = md.len();
    let sz_b = sz as f32;
    let sz_k = sz_b / 1024.0;
    let sz_m = sz_k / 1024.0;

    return sz_m; 
}



pub fn get_file_type_r(dir: &str) -> &str
{

    if check_ext_err(dir)
    {
        return "None";
    }

    let res = fs::metadata(dir);
    let md = res.unwrap();



    if md.file_type().is_symlink()
    {
        return "shortcut";
    }
    else if md.file_type().is_dir()
    {
        return "directory";
    }
    else if md.file_type().is_file()
    {
        //println!("{:?}", dir);
        let exten =  Path::new(dir).extension();
        if exten == None
        {
            return "None";
        }
        else
        {
            let ref_exten = exten.unwrap().to_str();
            return ref_exten.unwrap();
        }

        /* 
        if Ok(exten) 
        {

        }
        unwrap_or_else(|e| {return "None";});
        
        
            //println!("{:?}", exten);
        //.unwrap().to_str();
        //return exten.unwrap();
        return "dd";
        */

        //println!("{:?}", exten);
    }
    else
    {
        return "unknown";
    }
    
}

pub fn get_modified_r(dir: &str) -> String
{
    if check_ext_err(dir)
    {
        return "None".to_string();
    }

    let res = fs::metadata(dir);
    let md = res.unwrap();

    if let Ok(time_mod) = md.modified()
    {
        return iso8601(&time_mod);
    } 
    else 
    {
        return "99999999999".to_string();
    };
}

pub fn get_accessed_r(dir: &str) -> String
{
    if check_ext_err(dir)
    {
        return "None".to_string();
    }

    let res = fs::metadata(dir);
    let md = res.unwrap();

    if let Ok(time_acc) = md.accessed()
    {
        return iso8601(&time_acc);
    } 
    else 
    {
        return "99999999999".to_string();
    };
}

pub fn get_created_r(dir: &str) -> String
{
    if check_ext_err(dir)
    {
        return "None".to_string();
    }

    let res = fs::metadata(dir);
    let md = res.unwrap();


    if let Ok(time_cr) = md.created()
    {
        return iso8601(&time_cr);
    } 
    else 
    {
        return "99999999999".to_string();
    };

}


// get metadata formatted end ----------------------------------------------------------------------------------------------------------



// file manipulation start ----------------------------------------------------------------------------------------------------------





// file manipulation end ----------------------------------------------------------------------------------------------------------


// functionalities start ----------------------------------------------------------------------------------------------------------

pub fn store_initial_info(root_dir: &str,  lru: &mut Vec<fileNode>, largest:  &mut Vec<fileNode>, data_list: &mut Vec<fileNode>)
{

    for file in WalkDir::new(root_dir).into_iter().filter_map(|file| file.ok())
    {
        let ref_dir = file.path().to_str().unwrap();

        if check_ext_err(ref_dir)
        {
            continue;
        }
        //println!("{:?}", ref_dir);


        let ext = get_file_type_r(ref_dir);
        let sz_k = get_size_r(ref_dir) as u32;
        let modif = get_modified_r(ref_dir);
        let acc = get_accessed_r(ref_dir);
        let cr = get_created_r(ref_dir);

        //println!("{:?}", modif);
        //println!("{:?}", acc);
        //println!("{:?}", cr);

        let curr_file = fileNode::new(sz_k, acc.to_string(), modif.to_string(), cr.to_string(), ext.to_string(), ref_dir.to_string());
        let copy_curr_file = curr_file.clone();
        let copy2_curr_file = curr_file.clone();

        if curr_file.size >= 1     
        {
            data_list.push(curr_file);
            if largest.len() <= 50
            {
                largest.push(copy_curr_file);
            }
            else 
            {
                largest.sort();

                if copy_curr_file.size > largest[0].size
                {
                    largest.remove(0);
                    largest.push(copy_curr_file);
                }
            }

            if lru.len() <= 50
            {
                lru.push(copy2_curr_file);
            }
            else 
            {
                lru.sort_by(|a, b| b.modif.cmp(&a.modif));


                if copy2_curr_file.modif < lru[49].modif
                {
                    lru.remove(49);
                    lru.push(copy2_curr_file);
                }
            }
        }
    }
}


pub fn plot_overall_usage() -> (Vec<String>, Vec<f64>, Vec<f64>, f64, f64)
{
    let mut res = (vec![], vec![], vec![], 0.0, 0.0);
    //let mut names = Vec::<String>::default();
    //let mut tss = Vec::<f64>::default();
    //let mut tss = Vec::<f64>::default();


    let mut sys = System::new_all();

    let mut tot_agg_space: f64 = 0.0;
    let mut available_agg_space: f64 = 0.0;

    println!("=> disks:");
    for disk in sys.disks() {
    let nm = disk.name().to_str().unwrap().to_string();
    let ts = disk.total_space() as f64 / 1024.0 / 1024.0;
    let fs = disk.available_space() as f64 / 1024.0 / 1024.0;

    res.0.push(nm);
    res.1.push(ts);
    res.2.push(fs);

    tot_agg_space += ts;
    available_agg_space += fs;

    println!("name of disk is {:?}", disk.name());
    println!("total disk space is {:?} Mb", ts);
    println!("available space is {:?}", fs);
    println!("free percentage space is {:?} %", (fs / ts) * 100.0);
    }

    res.3 = tot_agg_space - available_agg_space;
    res.4 = available_agg_space; 
    return res;



}

// functionalities end ----------------------------------------------------------------------------------------------------------







// general testing ----------------------------------------------------------------------------------------------------------------------

pub fn get_all_info(dir: &str)
{
    for file in WalkDir::new(dir).into_iter().filter_map(|file| file.ok())
    {
        let ref_dir = file.path().to_str().unwrap();
        println!("{:?}", file.path().to_str().unwrap());
        get_file_type(ref_dir);
        get_size(ref_dir);
        get_modified(ref_dir);
        get_accessed(ref_dir);
        get_created(ref_dir);
    }
}



/*
pub fn get_file_type_n(dir: &str)
{
    for file in WalkDir::new(dir).into_iter().filter_map(|file| file.ok())
    {
        println!("{:?}", file.path().to_str().unwrap());
        let result = tree_magic::from_filepath(dir);
        println!("{:?}", result);
    }
}
*/

//----------------------------------------------------------------------------------------------------

use super::{MyViewF, SortFiles};
use crate::gui::styles;
use crate::gui::styles::colors::*;
use fltk::{app::MouseButton, enums::*, prelude::*, *};
use parking_lot::Mutex;
use std::str::FromStr;
use std::sync::atomic::Ordering;
use std::sync::Arc;
/*
use sysinfo::ProcessExt;
use sysinfo::System;
use sysinfo::SystemExt;
*/

/* 
struct ProcToggle {
    b: button::RadioButton,
}

impl ProcToggle {
    pub fn new(label: &str, ord: Arc<Mutex<SortOrder>>) -> Self 
    {
        let mut b = button::RadioButton::default()
            .with_size(70, 0)
            .with_label(label)
            .with_align(Align::Left | Align::Inside);

        b.set_down_frame(FrameType::FlatBox);
        b.set_selection_color(Color::color_average(b.color(), Color::Foreground, 0.9));
        b.clear_visible_focus();
        b.set_label_size(app::font_size() - 2);
        b.set_frame(FrameType::FlatBox);

        Self { b }
    }
}
*/


struct ProcToggleF {
    c: button::RadioButton,
}

impl ProcToggleF {
    pub fn new(label: &str, ord: Arc<Mutex<SortFiles>>) -> Self 
    {
        let mut c = button::RadioButton::default()
            .with_size(100, 0)
            .with_label(label)
            .with_align(Align::Left | Align::Inside);

        c.set_down_frame(FrameType::FlatBox);
        c.set_selection_color(Color::color_average(c.color(), Color::Foreground, 0.9));
        c.clear_visible_focus();
        c.set_label_size(app::font_size() - 2);
        c.set_frame(FrameType::FlatBox);

        Self { c }
    }
}


//fltk::widget_extends!(ProcToggle, button::RadioButton, b);
fltk::widget_extends!(ProcToggleF, button::RadioButton, c);



#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct fileNode
{
    // size in kb
    pub size: u32,
    //last accessed
    pub acc: String,
    pub modif: String,
    pub cre: String,
    // parent dir
    //parent: u32,
    // type of file
    pub tp: String,
    pub dir: String,
    // last modified, accessed, created 
}


impl fileNode
{
    pub fn new(size: u32, acc: String, modif: String, cre: String, tp: String, dir: String) -> Self {
        fileNode { size, acc, modif, cre, tp, dir }
    }

    pub fn fmt(&self, light: bool) -> String {
        if !light {
            format!(
                "@C255 {}\t@C255 {:}\t@C255 {:}\t@C255 {:}\t@C255 {:}\t@C255 {}",
                self.size,
                self.acc,
                self.modif,
                self.cre,
                self.tp,
                self.dir,
            )
        } else {
            format!(
                " {}\t\t {}\t\t {}\t\t {}\t\t{}\t\t{}",
                self.size,
                self.acc,
                self.modif,
                self.cre,
                self.tp,
                self.dir,
            )
        }
    }

}


/*
pub fn procs(view: &MyView) -> Option<Box<dyn FnMut() + Send>> {
    let mut ord = view.ordering.lock();
    *ord = SortOrder::Pid;
    drop(ord);

    let mut sys = view.system.lock();
    sys.refresh_processes();

    let grp = group::Pack::default()
        .with_size(700, 500)
        .center_of_parent();

    let hpack = group::Pack::default()
        .with_size(0, 30)
        .with_type(group::PackType::Horizontal);

    let mut b = ProcToggle::new("pid", view.ordering.clone());

    b.set_value(true);
    b.handle({
        let ord = view.ordering.clone();
        move |_, e| {
            if e == Event::Push {
                let mut ord = ord.lock();
                if *ord == SortOrder::Pid {
                    *ord = SortOrder::RevPid;
                } else {
                    *ord = SortOrder::Pid;
                }
                true
            } else {
                false
            }
        }
    });
   
*/

pub fn files(view: &MyViewF) -> Option<Box<dyn FnMut() + Send>> 
{

    let mut ord = view.ordering.lock();
    *ord = SortFiles::size;
    drop(ord);

    let mut sys = view.system.lock();
    sys.refresh_processes();

    let grp = group::Pack::default()
        .with_size(700, 500)
        .center_of_parent();

    let hpack = group::Pack::default()
        .with_size(0, 30)
        .with_type(group::PackType::Horizontal);

    let mut c = ProcToggleF::new("size (Mb)", view.ordering.clone());

    c.set_value(true);

    c.handle({
        let ord = view.ordering.clone();
        move |_, e| {
            if e == Event::Push
             {
                let mut ord = ord.lock();

                if *ord == SortFiles::size{
                    *ord = SortFiles::revSize;
                } else {
                    *ord = SortFiles::size;
                }
                return true
            } 
            else
            {
                return false
            }
        }


    });



    ProcToggleF::new("accessed", view.ordering.clone())
    .handle(
        {
        let ord = view.ordering.clone();
        move |_, e| {
            if e == Event::Push {
                let mut ord = ord.lock();
                if *ord == SortFiles::acc {
                    *ord = SortFiles::revAcc;
                } else {
                    *ord = SortFiles::acc;
                }
                true
            } else {
                false
            }
        }
    });
    
    
    ProcToggleF::new("modified", view.ordering.clone())
    .handle({
        let ord = view.ordering.clone();
        move |_, e| {
            if e == Event::Push {
                let mut ord = ord.lock();
                if *ord == SortFiles::modif {
                    *ord = SortFiles::revModif;
                } else {
                    *ord = SortFiles::modif;
                }
                true
            } else {
                false
            }
        }
    });

    ProcToggleF::new("created", view.ordering.clone())
    .handle({
        let ord = view.ordering.clone();
        move |_, e| {
            if e == Event::Push {
                let mut ord = ord.lock();
                if *ord == SortFiles::cr {
                    *ord = SortFiles::revCr;
                } else {
                    *ord = SortFiles::cr;
                }
                true
            } else {
                false
            }
        }
    });


    ProcToggleF::new("type", view.ordering.clone())
    .handle({
        let ord = view.ordering.clone();
        move |_, e| {
            if e == Event::Push {
                let mut ord = ord.lock();
                if *ord == SortFiles::typ {
                    *ord = SortFiles::revTyp;
                } else {
                    *ord = SortFiles::typ;
                }
                true
            } else {
                false
            }
        }
    });


    ProcToggleF::new("dir", view.ordering.clone())
    .handle({
        let ord = view.ordering.clone();
        move |_, e| {
            if e == Event::Push {
                let mut ord = ord.lock();
                if *ord == SortFiles::dir {
                    *ord = SortFiles::revDir;
                } else {
                    *ord = SortFiles::dir;
                }
                true
            } else {
                false
            }
        }
    });


    hpack.end();



    let mut c = browser::HoldBrowser::default().with_size(0, 500 - 30);
    c.clear_visible_focus();
    c.set_text_size(app::font_size() - 2);
    c.set_color(Color::color_average(c.color(), Color::Background, 0.1));
    c.set_selection_color(SEL_BLUE);
    c.set_scrollbar_size(5);
    c.scrollbar();
    c.set_selection_color(Color::color_average(c.color(), Color::Foreground, 0.9));
    c.hscrollbar();
    c.set_selection_color(Color::color_average(c.color(), Color::Foreground, 0.9));
    c.set_frame(FrameType::GtkDownBox);
    let widths = &[100, 100, 100, 100, 100];
    c.set_column_widths(widths);
    c.set_column_char('\t');



    let lru: &mut Vec<fileNode> = &mut Vec::<fileNode>::with_capacity(50);
    let largest: & mut Vec<fileNode> = &mut Vec::<fileNode>::with_capacity(50);
    let mut data_list: & mut Vec<fileNode> = &mut Vec::<fileNode>::default();
    let mut data_list2 = data_list.clone();
    store_initial_info("/home/mina/",  lru, largest, data_list);




    data_list.sort_by(|p1, p2| p1.size.cmp(&p2.size));

    let light_mode = view.light_mode.load(Ordering::Relaxed);

    for f in data_list {
        c.add(&f.fmt(light_mode));
    }


/* 
    let mut ps = vec![];
    for (pid, process) in sys.processes() {
        ps.push(Proc::new(pid, process));
    }



    ps.sort_by(|p1, p2| p1.size.cmp(&p2.pid));
    let light_mode = view.light_mode.load(Ordering::Relaxed);
    for p in ps {
        c.add(&p.fmt(light_mode));
    }
*/

// add the remove file functionality here


    let mut menu = menu::MenuButton::default().with_type(menu::MenuButtonType::Popup3);
    menu.set_frame(FrameType::FlatBox);
    menu.set_text_size(app::font_size() - 2);
    menu.set_color(Color::color_average(menu.color(), Color::Background, 0.9));

    menu.add("Remove file\t\t", Shortcut::None, menu::MenuFlag::Normal, {
        let sys = view.system.clone();
        let c = c.clone();
        move |_| {
            let val = c.value();
            if val != 0 {
                if let Some(text) = c.text(val) 
                
                {
                    let sys = sys.lock();
                    let v: Vec<&str> = text.split_ascii_whitespace().collect();
                    //println!("v is {:?}", v);
                    let dir = if light_mode {
                        v[5]
                    } else {
                        v[11]
                    };

                    //println!("dir is {:?}", dir);
                    delete_file(dir);
                    //println!("c is {:?}", c);
                    drop(sys);
                }


            }
        }
    });







    c.set_callback(move |_| {
        if app::event_mouse_button() == MouseButton::Right {
            menu.popup();
        }
    });


    let sys = Arc::new(Mutex::new(System::new_all()));
    let light_mode = view.light_mode.clone();
    let ord = view.ordering.clone();

    let cb = move || {

        if let Some(mut sys) = sys.try_lock(){

            let lru: &mut Vec<fileNode> = &mut Vec::<fileNode>::with_capacity(50);
            let largest: & mut Vec<fileNode> = &mut Vec::<fileNode>::with_capacity(50);
            let mut data_list: & mut Vec<fileNode> = &mut Vec::<fileNode>::default();
            store_initial_info("/home/",  lru, largest, data_list);


            data_list.sort_by(|p1, p2| match *ord.lock() {
                SortFiles::size => p1.size.cmp(&p2.size),
                SortFiles::revSize => p2.size.cmp(&p1.size),

                SortFiles::acc => p1.acc.cmp(&p2.acc),
                SortFiles::revAcc => p2.acc.cmp(&p1.acc),

                SortFiles::modif => p1.modif.cmp(&p2.modif),
                SortFiles::revModif => p2.modif.cmp(&p1.modif),

                SortFiles::cr => p1.cre.cmp(&p2.cre),
                SortFiles::revCr => p2.cre.cmp(&p1.cre),

                SortFiles::typ => p1.tp.cmp(&p2.tp),
                SortFiles::revTyp => p2.tp.cmp(&p1.tp),

                SortFiles::dir => p1.dir.cmp(&p2.dir),
                SortFiles::revDir => p2.dir.cmp(&p1.dir),
            });


            let light_mode = light_mode.load(Ordering::Relaxed);
            for (i, p) in data_list.iter().enumerate() {
                c.set_text(i as i32 + 1, &p.fmt(light_mode));
            }

            app::awake();
        }


    };


    grp.end();
    Some(Box::new(cb))

    /* 

    let sys = Arc::new(Mutex::new(System::new_all()));
    let light_mode = view.light_mode.clone();
    let ord = view.ordering.clone();
    let cb = move || {
        if let Some(mut sys) = sys.try_lock() {
            sys.refresh_processes();
            let mut ps = vec![];
            for (pid, process) in sys.processes() {
                ps.push(Proc::new(pid, process));
            }

            ps.sort_by(|p1, p2| match *ord.lock() {
                SortOrder::Pid => p1.pid.cmp(&p2.pid),
                SortOrder::Mem => p1.memory.cmp(&p2.memory),
                SortOrder::Virt => p1.virt.cmp(&p2.virt),
                SortOrder::Cpu => p1.cpu.partial_cmp(&p2.cpu).unwrap(),
                SortOrder::Exe => p1.exe.cmp(&p2.exe),
                SortOrder::RevPid => p2.pid.cmp(&p1.pid),
                SortOrder::RevMem => p2.memory.cmp(&p1.memory),
                SortOrder::RevVirt => p2.virt.cmp(&p1.virt),
                SortOrder::RevCpu => p2.cpu.partial_cmp(&p1.cpu).unwrap(),
                SortOrder::RevExe => p2.exe.cmp(&p1.exe),
            });

            let light_mode = light_mode.load(Ordering::Relaxed);
            for (i, p) in ps.iter().enumerate() {
                b.set_text(i as i32 + 1, &p.fmt(light_mode));
            }
            app::awake();
        }
    };
    grp.end();
    Some(Box::new(cb))


*/
}
