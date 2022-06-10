#![feature(path_file_prefix)]

use std::io::prelude::*;
use std::{fs::File, mem::size_of, thread, time::Duration};

use anyhow::*;
use goblin::pe::PE;
use rfd::{FileDialog, MessageButtons, MessageDialog};

pub type WORD = u16;
pub type DWORD = u32;
pub type LONG = i32;

#[repr(C)]
pub struct ImageDosHeader {
    pub e_magic: WORD,
    pub e_cblp: WORD,
    pub e_cp: WORD,
    pub e_crlc: WORD,
    pub e_cparhdr: WORD,
    pub e_minalloc: WORD,
    pub e_maxalloc: WORD,
    pub e_ss: WORD,
    pub e_sp: WORD,
    pub e_csum: WORD,
    pub e_ip: WORD,
    pub e_cs: WORD,
    pub e_lfarlc: WORD,
    pub e_ovno: WORD,
    pub e_res: [WORD; 4],
    pub e_oemid: WORD,
    pub e_oeminfo: WORD,
    pub e_res2: [WORD; 10],
    pub e_lfanew: LONG,
}

#[repr(C)]
pub struct ImageFileHeader {
    pub machine: WORD,
    pub number_of_sections: WORD,
    pub time_date_stamp: DWORD,
    pub pointer_to_symbol_table: DWORD,
    pub number_of_symbols: DWORD,
    pub size_of_optional_header: WORD,
    pub characteristics: WORD,
}

#[repr(C)]
pub struct ImageNtHeaders {
    pub signature: DWORD,
    pub file_header: ImageFileHeader,
}

fn main() -> Result<()> {
    println!("Select the EXE file to patch.");
    thread::sleep(Duration::from_millis(1000));

    let files = FileDialog::new().add_filter("EXE", &["exe"]).pick_file();

    if files.is_none() {
        println!("Canceled. Exit after 2secs...");
        thread::sleep(Duration::from_millis(2000));
        return Ok(());
    }

    let mut src = files.unwrap();
    let exe_name = &src.file_stem().unwrap().to_string_lossy();
    let mut file = File::open(&src)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    PE::parse(&mut buffer).with_context(|| "Failed to parse. The PE file is invalid.")?;

    let addr = &mut buffer[0] as *mut u8;
    let dos_header = addr as *mut ImageDosHeader;
    let nt_header_start = unsafe { (*dos_header).e_lfanew };
    let offset = nt_header_start as usize + size_of::<DWORD>() + size_of::<ImageFileHeader>()
        - size_of::<WORD>();
    buffer[offset] = buffer[offset] | 0x0020;

    let is_separate = MessageDialog::new()
        .set_title("laa-patch-rs")
        .set_description("Do you want to output the patch as a separate EXE file?")
        .set_buttons(MessageButtons::YesNo)
        .show();

    if is_separate {
        src.set_file_name(format!("{}_patched.exe", exe_name));
    };

    let mut patched = File::create(src)?;
    patched.write(&buffer)?;
    patched.flush()?;

    println!("Patched. Exit after 2secs...");
    thread::sleep(Duration::from_millis(2000));
    Ok(())
}
