#![feature(proc_macro_hygiene)]
use std::{
    io,
    env,
    path::Path,
    convert::TryInto,
    str::FromStr,
    fs,
    thread::{self}
};
use arcropolis_api;

const IDENTIFIER: &str = "smashline_legs";

use std::sync::RwLock;
lazy_static! {
    static ref MOD_DIR: RwLock<String> = RwLock::new("".to_string());
}


fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    //std::fs::create_dir_all(&dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            //if entry.file_name().to_str().unwrap() == "vl.prcxml" {continue;}
            std::fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
            println!("[smashline_tantan::data] copied {} to {}",entry.file_name().to_str().unwrap(),dst.as_ref().to_str().unwrap());
        }
    }
    Ok(())
}

pub fn patch_files()
{
    unsafe {
        let motionFolder = format!("{}/fighter/tantan/motion/body/",&*MOD_DIR.read().unwrap());
        let slots=8;
        if !Path::new(motionFolder.as_str()).exists()
        {
            println!("[smashline_tantan::data] WTF?");
            return;
        }
        let file = "motion_list.motdiff";
        let sourceFolder = format!("{}/c00/",motionFolder.as_str());
        let sourceFile = format!("{}/c00/{}",motionFolder.as_str(),file);
        for slot in 1..slots {
            let buffer = if slot<10 {"0"} else {""};
            let destFolder = format!("{}/c{}{}",motionFolder.as_str(),buffer,slot);
            //fs::create_dir_all(destFolder.as_str());
            copy_dir_all(sourceFolder.as_str(),destFolder.as_str());
            println!("[smashline_tantan::data] copied motion files to {}",destFolder);
        }

    }
    
}

pub fn inital_setup()->bool {
    let mut found_folder = false;

    unsafe {
        for entry in std::fs::read_dir("sd:/ultimate/mods").unwrap() {
            let entry = entry.unwrap();
            let mut path = entry.path();
            if path.is_dir() {
                path.push(IDENTIFIER);
                if Path::new(&path).exists() {
                    found_folder = true;
                    path.pop();
                    *MOD_DIR.write().unwrap() = format!("{}", path.display());
                    break;
                }
            }
        }
    }
    return found_folder;
}


pub fn install() {
    if inital_setup() {
        let install_thread = std::thread::spawn(move || {
            patch_files();
        });
        install_thread.join();
    }
}