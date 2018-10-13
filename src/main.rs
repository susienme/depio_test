extern crate depio;

use std::env;
use std::fs::File;
use std::io::Read;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

fn main() {

	/*
	 * Just read data from a file to memory
	 * You can ignore this part
	 */
    let mut args = env::args();
    if args.len() != 2 {return;}
    args.next();
    let pathname = args.next().unwrap();
    let mut buf = Vec::new();
    match File::open(&pathname) {
        Ok(mut f) => {
            assert!(f.read_to_end(&mut buf).unwrap() > 0);
        }
        Err(e) => {
            println!("Error with file `{}' -- {:?}", pathname, e);
            return;
        }
    }
    let mem: &str;
    mem = std::str::from_utf8(&buf).unwrap();

    /*
     * EXAMPLE
     * mem: &str
     * deps: Some<BTreeMap<String, BTreeSet<String>>>
     */
	let deps = depio::read_from_mem(mem);

	if let Some(_deps) = deps {
		print_pretty(&_deps);
	}

 }

fn print_pretty(deps: &BTreeMap<String, BTreeSet<String>>) {
	for (dependent, to_whom) in deps {
		println!("{} depends on:", dependent);
		for whom in to_whom {
			println!("    {}", whom);
		}
	}
}