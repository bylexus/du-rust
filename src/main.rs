use du_rust::config::Config;
use du_rust::config::PathList;
use du_rust::DirInfo;
use du_rust::FSItem;
use du_rust::FSItemList;
use du_rust::FileInfo;
use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

fn examine_file(file: &mut FileInfo) -> Result<(), io::Error> {
    file.size = file.path.metadata()?.len();
    Ok(())
}

fn examine_dir(dir: &mut DirInfo) -> Result<(), io::Error> {
    let content = fs::read_dir(&dir.path)?;
    for f in content {
        if let Ok(dir_entry) = f {
            match examine_path(&dir_entry.path()) {
                Ok(item) => {
                    dir.size += match &item {
                        FSItem::Dir(d) => d.size,
                        FSItem::File(f) => f.size,
                    };
                    dir.childs.push(item);
                }
                Err(_) => {
                    eprintln!("Cannot examine dir {:?}", dir.path);
                }
            }
        } else {
            continue;
        }
    }
    return Ok(());
}

fn examine_path(path: &PathBuf) -> Result<FSItem, io::Error> {
    if path.is_dir() {
        let mut dir = DirInfo::new(path);
        let res = examine_dir(&mut dir);
        return match res {
            Ok(()) => Ok(FSItem::Dir(dir)),
            Err(err) => Err(err),
        };
    } else if path.is_file() {
        let mut file = FileInfo::new(path);
        let res = examine_file(&mut file);
        return match res {
            Ok(()) => Ok(FSItem::File(file)),
            Err(err) => Err(err),
        };
    } else {
        return Err(io::Error::new(io::ErrorKind::NotFound, ""));
    }
}

fn examine_paths(pl: &PathList) -> FSItemList {
    let mut list = FSItemList::new();
    for path in pl {
        let res = examine_path(path);
        if let Ok(item) = res {
            list.push(item);
        }
    }
    return list;
}

fn output_items(items: &FSItemList) {
    for i in items {
        let path = match &i {
            FSItem::File(f) => f.path.to_str().unwrap_or("ERROR"),
            FSItem::Dir(d) => d.path.to_str().unwrap_or("ERROR"),
        };
        let size = match &i {
            FSItem::File(f) => f.size,
            FSItem::Dir(d) => d.size,
        };
        println!("{} {}", size, path);
    }
}

fn main() {
    let mut fileargs: Vec<String> = env::args().collect();
    fileargs.remove(0);
    let c = Config::new(&fileargs);

    let path_infos = examine_paths(&c.filenames);
    output_items(&path_infos);

    return;

    // let what = fs::read_dir(Path::new("/Users/alex/dev")).expect("Moo");
}
