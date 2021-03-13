use std::fs::File;
use std::io::SeekFrom;
use std::path::Path;


fn main() {
    println!("idRehash 1.0 Rust rewrite by leveste. Based on the original file by infogram and Proteh.\n");

    if !Path::new("meta.resources").exists(){
        println!("ERROR: meta.resources not found.");
        std::process::exit(1);
    }

    let meta = File::open("meta.resources");

    //meta.seek(SeekFrom::Start(0x50));
}
