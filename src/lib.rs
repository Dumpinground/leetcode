#[path ="libs/1.两数之和.rs"]
pub mod s1;

#[path ="libs/13.罗马数字转整数.rs"]
pub mod s13;

#[path ="libs/14.最长公共前缀.rs"]
pub mod s14;

use std::path::Path;
use std::{fs, io};
use std::fs::DirEntry;

fn visit_dir(dir: &Path) -> io::Result<Vec<String>> {
    let mut paths = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path().into_os_string();
            let path = String::from(path.to_str().unwrap());
            paths.push(path);
        }
    }
    Ok(paths)
}

#[test]
fn pathfinder() {
    let path = Path::new("libs");
    let paths = visit_dir(path).unwrap();
    for p in paths {
        println!("{}", p);
    }
    println!("pathfinder");
}
