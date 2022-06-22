use std::ffi::OsString;
use std::io::{BufWriter, Write, stdout};
use std::os::unix::prelude::{MetadataExt, PermissionsExt};
use std::{thread, time};
use std::{fs, path::Path};
use colored::Colorize;
use users::{UsersCache, Users, Groups};


fn build_list(raw_path: &Path, cache: &UsersCache) -> OsString {
    let dir = fs::read_dir(raw_path)
                        .expect("It is impossibile to read thi Directory...");
    let mut list_string = OsString::new();

    for path in dir {
        
        let p = match path {
            Ok(path) => path.path(),
            Err(_) => continue
        };

        let file_name = match p.file_name() {
            Some(name) => String::from(match name.to_str() {
                Some(name) => name,
                None => continue
            }),
            None => continue
        };

        let meta = match p.metadata() {
            Ok(meta) => meta,
            Err(_) => continue
        };

        let m: u32 = meta.permissions().mode();
        let is_dir = meta.is_dir();
        let uid = meta.uid();
        let gid = meta.gid();
        let mut count_x: u8 = 0;

        list_string.push(if is_dir {"d"}else{"-"});
        list_string.push(if m & (0x1<<8) >= 1 {"r"}else{"-"});
        list_string.push(if m & (0x1<<7) >= 1 {"w"}else{"-"});
        list_string.push(
            if m & (0x1<<6) >= 1 {
                count_x += 1;
                "x"
            } else {"-"}
        );
        list_string.push(if m & (0x1<<5) >= 1 {"r"}else{"-"});
        list_string.push(if m & (0x1<<4) >= 1 {"w"}else{"-"});
        list_string.push(
            if m & (0x1<<3) >= 1 {
                count_x += 1;
                "x"
            } else {"-"}
        );
        list_string.push(if m & (0x1<<2) >= 1 {"r"}else{"-"});
        list_string.push(if m & (0x1<<1) >= 1 {"w"}else{"-"});
        list_string.push(
            if m & 0x1 >= 1 {
                count_x += 1;
                "x"
            } else {"-"}
        );
 
        list_string.push(" ");

        if let Some(user) = cache.get_user_by_uid(uid) {
            list_string.push(user.name().to_os_string());
        } else {
            list_string.push(uid.to_string());
        }
        list_string.push(" ");

        if let Some(group) = cache.get_group_by_gid(gid) {
            list_string.push(group.name().to_os_string());
        }
        else {
            list_string.push(gid.to_string());
        }
        list_string.push(" ");

        list_string.push(meta.len().to_string());
        list_string.push(" ");
        if is_dir {
            list_string.push(file_name.blue().bold().to_string());
            list_string.push("/");
        }
        else {
            if count_x == 3 {
                list_string.push(file_name.green().bold().to_string());
                list_string.push("*");
            } 
            else {
                list_string.push(file_name);
            }
        }
        list_string.push("\n");
    }
    list_string
}


fn main() {
    let path = Path::new("/home/gg/Scrivania");
    let millis = time::Duration::from_millis(100);
    let mut cache = UsersCache::new();

    let mut main_string = build_list(path, &mut cache);
    print!("{esc}c", esc = 27 as char);
    println!("{}", main_string.to_str().expect("It is impossible to read the result"));

    loop {
        thread::sleep(millis);

        let new_string = build_list(path, &mut cache);

        if main_string != new_string {
            main_string = new_string;

            let mut buf = BufWriter::new(stdout());
            write!(buf, "{esc}c", esc = 27 as char).unwrap(); // clear terminal
            write!(buf, "{}", main_string.to_str().unwrap()).unwrap();
            buf.flush().unwrap();
        }
    }
    
}
