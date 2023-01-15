use std::{fs, string};
use std::path::Path;

extern crate walkdir;
use walkdir::WalkDir;


use chrono::prelude::{DateTime, Utc};

use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};
use sysinfo::DiskExt;


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
        return unix_time(&time_mod);
    } 
    else 
    {
        return "999999999999999".to_string();
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
        return unix_time(&time_acc);
    } 
    else 
    {
        return "999999999999999".to_string();
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
        return unix_time(&time_cr);
    } 
    else 
    {
        return "999999999999999".to_string();
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

        let curr_file = fileNode::new(sz_k, acc.to_string(), modif.to_string(), cr.to_string(), ext.to_string(), ref_dir.to_string());
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