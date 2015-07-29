use std::io::prelude::*;
use std::io::{self, BufReader};
use std::env;
use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::fmt::{self, Display, Formatter};

struct LocContext {
    files: usize,
    lines: usize,
}

impl LocContext {
    fn new() -> LocContext {
        LocContext{ files: 0, lines: 0 }
    }
}

impl Display for LocContext {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "LocContext {{ files: {}, lines: {} }}", self.files, self.lines)
    }
}

fn visit_file(file: &Path, context: &mut LocContext) -> io::Result<()> {
    if file.extension().map_or(false, |ext| ext == "rs") {
        println!("visit_file: {}", file.display());
        let buf = BufReader::new(try!(File::open(file)));
        context.lines += buf.lines().count();
        context.files += 1;
    }
    Ok(())
}

// path maybe a file or dir
fn visit_path(path: &Path, context: &mut LocContext) -> io::Result<()> {
    let metadata = try!(fs::metadata(path));
    if metadata.is_file() {
        return visit_file(path, context);
    }

    for entry in try!(fs::read_dir(path)) {
        let entry = try!(entry);
        let entry_type = try!(entry.file_type());
        if entry_type.is_dir() {
            try!(visit_path(&entry.path(), context));
        } else if entry_type.is_file() {
            try!(visit_file(&entry.path(), context));
        } else {
            unimplemented!();
        }
    }
    Ok(())
}

fn main() {
    let mut args = env::args();
    args.next();
    let path: PathBuf = if let Some(arg1) = args.next() {
        PathBuf::from(arg1)
    } else {
        env::current_dir().unwrap()
    };
    println!("path: {:?}", path);

    let mut context = LocContext::new();
    match visit_path(&path, &mut context) {
        Ok(_) => {
            println!("{}", context);
        }
        Err(err) => println!("{}", err),
    }
}
