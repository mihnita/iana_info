use std::time::{SystemTime};
use std::collections::BTreeMap;
use std::io::{self, Write, Read};
use std::fs::{self, File};
use std::path::{Path};
use serde::ser::{Serialize, Serializer, SerializeStruct};

pub struct FileInfo {
    pub path: String,
    pub size: u64,
    pub crc32: String,
}

impl Serialize for FileInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("FileInfo", 3)?;
        state.serialize_field("path", &self.path)?;
        state.serialize_field("size", &self.size)?;
        state.serialize_field("crc32", &self.crc32)?;
        state.end()
    }
}

// fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
//     cb(&entry);
// }

fn crc32(path: &Path) -> u32 {
    let mut hasher = crc32fast::Hasher::new();

    let mut file = File::open(path).unwrap();
    let mut buffer = [0; 102400];
    loop {
        let read_len = file.read(&mut buffer).unwrap();
        if read_len == 0 {
            break;
        }
        let x = buffer.split_at(read_len);
        hasher.update(&x.0);
    }
    // hasher.update(b"foo bar baz");
    hasher.finalize()
}

fn visit_dirs(dir: &Path, db: &mut BTreeMap<String, FileInfo>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, db)?;
            } else {
                let crc: u32 = crc32(&path);
                let name = path.to_string_lossy().replace("\\", "/");
                let fi: FileInfo = FileInfo{
                    path: name.clone(),
                    size: entry.metadata()?.len(),
                    crc32: format!("{:08x}", crc)
                };
                db.insert(name, fi);
            }
        }
    }
    Ok(())
}

fn scan_folders(path: &Path) -> BTreeMap<String, FileInfo> {
    let mut db: BTreeMap<String, FileInfo> = BTreeMap::new();
    let _rez = visit_dirs(path, &mut db);
    db
}

// LOADING:
// let obj: Map<String, Value> = parsed.as_object().unwrap().clone();

fn main() {
    let start = SystemTime::now();
    let path = Path::new("M:/Books_calibre/AmazonUntouched");
    let db: BTreeMap<String, FileInfo> = scan_folders(path);
    let end = SystemTime::now();

    let duration = end.duration_since(start);
    println!("Time: {:?}", duration.unwrap());

    let json: String = serde_json::to_string_pretty(&db).unwrap();
    let mut file = File::create("foo.json").unwrap();
    let result = file.write_all(json.as_bytes());
    if result.is_err() {
        println!("ERROR!");
    } else {
        println!("DONE!");
    }
    // println!("{:?}", json);
}
