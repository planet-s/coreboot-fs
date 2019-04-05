use coreboot_fs::Rom;
use std::{env, fs, io, process};

fn inner(path: &str) -> io::Result<()> {
    let data = fs::read(path)?;
    let rom = Rom::new(&data);

    if let Some(fmap) = rom.fmap() {
        let mut name = String::new();
        for &b in fmap.name.iter() {
            if b == 0 {
                break;
            }
            name.push(b as char);
        }

        eprintln!("  {}", name);

        for i in 0..fmap.nareas {
            let area = fmap.area(i);

            let mut name = String::new();
            for &b in area.name.iter() {
                if b == 0 {
                    break;
                }
                name.push(b as char);
            }

            eprintln!("    {}: {}", i, name);
        }
    }

    if let Some(header) = rom.header() {
        eprintln!("  CBFS");

        let align = u32::from(header.align) as usize;
        let mut offset = u32::from(header.offset) as usize;
        while let Some(file) = rom.file(offset) {
            let name_bytes = file.name();

            let mut name = String::new();
            for &b in name_bytes.iter() {
                if b == 0 {
                    break;
                }
                name.push(b as char);
            }

            eprintln!("    {:#X}: {}", offset, name);

            let file_offset = u32::from(file.offset) as usize;
            let file_len = u32::from(file.len) as usize;
            offset = ((offset + file_offset + file_len + align - 1) / align) * align;
        }
    } else {
        eprintln!("coreboot header not found");
    }

    Ok(())
}

fn main() {
    for path in env::args().skip(1) {
        eprintln!("{}", path);
        if let Err(err) = inner(&path) {
            eprintln!("failed to parse {}: {}", path, err);
            process::exit(1);
        }
    }
}
