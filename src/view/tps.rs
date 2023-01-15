
use std::{fs, string};
use std::path::Path;

extern crate walkdir;
use walkdir::WalkDir;


use chrono::prelude::{DateTime, Utc};

use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};
use sysinfo::DiskExt;

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

    pub kind: String
    // last modified, accessed, created 
}



impl fileNode
{
    pub fn new(size: u32, acc: String, modif: String, cre: String, tp: String, dir: String, kind: String) -> Self {
        fileNode { size, acc, modif, cre, tp, dir, kind}
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
    if exten == None || exten.unwrap() == "ini" || exten.unwrap() == "so" || exten.unwrap() == "gz" || exten.unwrap() == "md"
    || exten.unwrap() == "page" 
    {
        return true;
    }
    else 
    {
        return false;
    }
}

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

    return sz_k; 
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

        let knd = get_kind(ext.clone());

        //println!("size {:?}", sz_k.clone() );
        //println!("kind {:?}", knd);

        let curr_file = fileNode::new(sz_k, acc.to_string(), modif.to_string(), cr.to_string(), ext.to_string(), ref_dir.to_string(), knd.to_string());
        let copy_curr_file = curr_file.clone();
        let copy2_curr_file = curr_file.clone();

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

fn get_kind(ext: &str) -> &str
{
 

let aud_type: [&str;14] = 
[
    "mp3",
    "aac",
    "mpa",
    "wma",
    "wpl",
    "ogg",
    "wav",
    "alac",
    "flac",
    "aiff",
    "aif",
    "dsd",
    "pcm",
    "m4a"];



let mut comp_type: [&str;9 ] = [
"zip",
"rar",
"7z",
"arj",
"deb",
"pkj",
"rpm",
"tar.gz",
"z"
];

let mut text_type: [&str;12 ] = [
"pdf",
"doc",
"docx",
"html",
"htm",
"xls",
"xlsx",
"txt",
"wpd",
"tex",
"rtf",
"odt"
];

let mut img_type: [&str;12] = 
[
"gif",
"png",
"jpg",
"jpeg",
"svg",
"png",
"tiff",
"tif",
"ps",
"psd",
"ico",
"bmp"
];



let mut vid_type: [&str;30 ] = [
"webm",
"mkv",
"flv",
"f4v",
"swf",
"vob",
"ogv",
"drc",
"gifv",
"mng",
"avi",
"mov",
"wmv",
"mp4",
"m4p",
"yuv",
"avchd",
"rm",
"rmbv",
"viv",
"asf",
"amv",
"m4v",
"mpg",
"mpeg-2",
"mpeg",
"m2v",
"m4v",
"svi",
"3gp"
];

for t in vid_type {
    if t == (ext)
        {return "video"}
}

for t in comp_type {
    if t == (ext)
        {return "compressed"}
}

for t in aud_type {
    if t == (ext)
        {return "audio"}
}

for t in text_type {
    if t == (ext)
        {return "text"}
}

for t in img_type {
    if t == (ext)
        {return "image"}
}

return "other"



}


    

//-----------------------------------------------------------------------------

use super::MyViewF;
use crate::{
    gui::styles::colors::*,
    gui::widgets::{Card, Dial, HalfDial},
};
use fltk::{prelude::*, *};
use parking_lot::Mutex;
use std::sync::Arc;


use fltk::{enums::*, prelude::*, *};




pub fn Types(view: &MyViewF) -> Option<Box<dyn FnMut() + Send>> {
    
    let mut sys = view.system.lock();
    sys.refresh_all();

    let lru: &mut Vec<fileNode> = &mut Vec::<fileNode>::with_capacity(50);
    let largest: & mut Vec<fileNode> = &mut Vec::<fileNode>::with_capacity(50);
    let mut data_list: & mut Vec<fileNode> = &mut Vec::<fileNode>::default();
    store_initial_info("/home/",  lru, largest, data_list);

    let mut tot_vid:f64 = 0.0;
    let mut tot_comp:f64 = 0.0;
    let mut tot_aud:f64   = 0.0;
    let mut tot_img:f64  = 0.0;
    let mut tot_txt:f64  = 0.0;
    let mut tot_other:f64 = 0.0;

    let mut tt = 0.0;

    for f in data_list
    {
        //println!("comp {:?}", tot_comp);
        /*
        println!("first {:?}", f.size);
        println!("second {:?}", f.size as f64);
        */
        tt += f.size as f64;
        if f.kind == "video" {tot_vid += f.size as f64;}
        else if f.kind == "compressed" {tot_comp += f.size as f64;}
        else if f.kind == "audio" {tot_aud += f.size as f64;}
        else if f.kind == "text" {tot_img += f.size as f64;}
        else if f.kind == "image" {tot_txt += f.size as f64;}
        else if f.kind == "other" {tot_other += f.size as f64;}
    }

    /*
    println!("tot {:?}", tt);
    println!("tot other {:?}", tot_other );
    println!("tot comp {:?}", tot_comp);
    println!("tot img {:?}", tot_img as f32 );
    println!("tot vid {:?}", tot_vid as f32 );
    println!("tot aud {:?}", tot_aud as f32 );
    println!("tot txt {:?}", tot_txt as f32 );
    */


    let mut dials = vec![];


    frame::Frame::new(60, 60, 0, 0, None);

    let mut grp = group::Pack::default()
        .with_size(700, 450)
        .center_of_parent();
    grp.set_spacing(30);

    let mut pack0 = group::Pack::default()
        .with_size(450, 200)
        .with_type(group::PackType::Horizontal);
    pack0.set_spacing(40);

    let mut dial = HalfDial::default().with_size(200, 200).with_label("Video %");
    dial.set_value(((tot_vid / tt) * (100 as f64)) as i32);
    dial.set_selection_color(CPU_GREEN);
    dials.push(dial);
    
    let mut dial = HalfDial::default()
        .with_size(200, 200)
        .with_label("Compressed %");
    dial.set_selection_color(MEM_YELLOW);
    dial.set_value(((tot_comp / tt) * (100 as f64)) as i32);
    dials.push(dial);


    let mut dial = HalfDial::default().with_size(200, 200).with_label("Audio %");
    dial.set_selection_color(DISK_PURPLE);
    dial.set_value(((tot_aud / tt) * (100 as f64)) as i32);
    dials.push(dial);
    pack0.end();

    let mut pack0 = group::Pack::default()
    .with_size(450, 200)
    .with_type(group::PackType::Horizontal);
    pack0.set_spacing(40);

    let mut dial = HalfDial::default().with_size(200, 200).with_label("Image %");
    dial.set_value(((tot_img / tt) * (100 as f64)) as i32);
    dial.set_selection_color(CPU_GREEN);
    dials.push(dial);

    let mut dial = HalfDial::default()
        .with_size(200, 200)
        .with_label("Text %");

    dial.set_selection_color(MEM_YELLOW);
    dial.set_value(((tot_txt / tt) * (100 as f64)) as i32);
    dials.push(dial);


    let mut dial = HalfDial::default().with_size(200, 200).with_label("Other %");
    dial.set_selection_color(DISK_PURPLE);
    dial.set_value(((tot_other / tt) * (100 as f64)) as i32);
    dials.push(dial);
    pack0.end();


    grp.end();



    drop(sys);
    let dials = Arc::new(Mutex::new(dials));
    let sys = Arc::new(Mutex::new(System::new_all()));
    let cb = move || {
        if let Some(mut sys) = sys.try_lock() {


            let lru: &mut Vec<fileNode> = &mut Vec::<fileNode>::with_capacity(50);
            let largest: & mut Vec<fileNode> = &mut Vec::<fileNode>::with_capacity(50);
            let mut data_list: & mut Vec<fileNode> = &mut Vec::<fileNode>::default();
            store_initial_info("/home/",  lru, largest, data_list);
        
            let mut tt  = 0.0;
            let mut tot_vid:f64 = 0.0;
            let mut tot_comp:f64 = 0.0;
            let mut tot_aud:f64   = 0.0;
            let mut tot_img:f64  = 0.0;
            let mut tot_txt:f64  = 0.0;
            let mut tot_other:f64 = 0.0;
        
            for f in data_list
            {
                tt += f.size as f64;
                if f.kind == "video" {tot_vid += f.size as f64;}
                else if f.kind == "compressed" {tot_comp += f.size as f64;}
                else if f.kind == "audio" {tot_aud += f.size as f64;}
                else if f.kind == "text" {tot_img += f.size as f64;}
                else if f.kind == "image" {tot_txt += f.size as f64;}
                else if f.kind == "other" {tot_other += f.size as f64;}
            }
        




            dials.lock()[0].set_value(((tot_vid / tt) * (100 as f64)) as i32);
            dials.lock()[1].set_value(((tot_comp / tt) * (100 as f64)) as i32);
            dials.lock()[2].set_value(((tot_aud / tt) * (100 as f64)) as i32);
            dials.lock()[3].set_value(((tot_img / tt) * (100 as f64)) as i32);
            dials.lock()[4].set_value(((tot_txt / tt) * (100 as f64)) as i32);
            dials.lock()[5].set_value(((tot_other / tt) * (100 as f64)) as i32);

            app::awake();
        }
    };
    Some(Box::new(cb))
}


