use std::io::{BufWriter, Write, stdout};
use std::os::unix::prelude::{MetadataExt, PermissionsExt};
use std::{thread, time, env};
use std::{fs, path::Path};
use colored::Colorize;
use users::{UsersCache, Users, Groups};


fn build_list(raw_path: &Box<Path>, cache: &UsersCache) -> String {
    let dir = fs::read_dir(raw_path)
                        .expect("It is impossibile to read thi Directory...");
    let mut list_string = String::with_capacity(100); // standard initial capacity, for now only 100

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

        list_string.push(if is_dir {'d'}else{'-'});
        list_string.push(if m & (0x1<<8) >= 1 {'r'}else{'-'});
        list_string.push(if m & (0x1<<7) >= 1 {'w'}else{'-'});
        list_string.push(
            if m & (0x1<<6) >= 1 {
                count_x += 1;
                'x'
            } else {'-'}
        );
        list_string.push(if m & (0x1<<5) >= 1 {'r'}else{'-'});
        list_string.push(if m & (0x1<<4) >= 1 {'w'}else{'-'});
        list_string.push(
            if m & (0x1<<3) >= 1 {
                count_x += 1;
                'x'
            } else {'-'}
        );
        list_string.push(if m & (0x1<<2) >= 1 {'r'}else{'-'});
        list_string.push(if m & (0x1<<1) >= 1 {'w'}else{'-'});
        list_string.push(
            if m & 0x1 >= 1 {
                count_x += 1;
                'x'
            } else {'-'}
        );
 
        list_string.push(' ');

        if let Some(user) = cache.get_user_by_uid(uid) {
            list_string.push_str(user.name().to_str().unwrap());
        } else {
            list_string.push_str(&uid.to_string());
        }
        list_string.push(' ');

        if let Some(group) = cache.get_group_by_gid(gid) {
            list_string.push_str(group.name().to_str().unwrap());
        }
        else {
            list_string.push_str(&gid.to_string());
        }
        list_string.push(' ');

        list_string.push_str(&meta.len().to_string());
        list_string.push(' ');
        if is_dir {
            list_string.push_str(&file_name.blue().bold().to_string());
            list_string.push('/');
        }
        else {
            if count_x == 3 {
                list_string.push_str(&file_name.green().bold().to_string());
                list_string.push('*');
            } 
            else {
                list_string.push_str(&file_name);
            }
        }
        list_string.push('\n');
    }
    list_string
}


/// Parse the arguments of the program.
///
/// There must be only one argument, a path to a directory.
/// Return the Path if exists and is a directory, otherwise return None. 
fn parse_arguments() -> Option<Box<Path>> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let path = Path::new(&args[1]);
        if !path.exists() {
            println!("Error. The path doesn't exists...");
            return None;
        }
        if !path.is_dir() {
            println!("Error. The path isn't a directory. Exit...");
            return None;
        }
        return Some(Box::from(path));
    }
    println!("Error. No argument. Exit...");
    None
}


fn main() {
    let path = match parse_arguments() {
        Some(path) => path,
        None => return
    };

    let millis = time::Duration::from_millis(100);
    let mut cache = UsersCache::new();

    let mut main_string = build_list(&path, &mut cache);
    print!("{esc}c", esc = 27 as char); // clear terminal
    println!("{}", main_string);

    loop {
        thread::sleep(millis);

        let new_string = build_list(&path, &mut cache);

        if main_string != new_string {
            main_string = new_string;

            let mut buf = BufWriter::new(stdout());
            write!(buf, "{esc}c", esc = 27 as char).unwrap(); // clear terminal
            write!(buf, "{}", main_string).unwrap();
            buf.flush().unwrap();
        }
    }
    
}
