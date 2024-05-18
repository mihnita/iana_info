use std::fs::File;

fn get_class_version(file_name: &str) {
    let mut f = File::open(file_name)?;
    let mut buffer = [0; 10];
    f.read(&mut buffer)?;
}

fn main() {
    println!("Hello, world!");

    let file_name = "/Users/mnita/third_party//icu.before_merge/icu4j/main/tests/collate/out/bin/com/ibm/icu/dev/test/util/ULocaleCollationTest.class";

    // {
        // let r = &mut f;
        // let mut buffer = vec![0_u8; 100];
        // r.take(100);
        // r.read(&mut buffer)?;
        // let mut rdr = Cursor::new(buffer);
        // let file_code = rdr.read_u32::<BigEndian>()?;
        // assert_eq!(9994_u32, file_code);
    // }
}
