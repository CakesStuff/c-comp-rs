use std::env::args;

unsafe extern "C" {
    fn cmain(argc: i32, argv: *const *const u8) -> i32;
}

fn main() {
    let args = args();
    let args = args.map(|arg| arg.as_bytes().as_ptr()).collect::<Vec<*const u8>>();
    std::process::exit(unsafe { cmain(args.len() as i32, args.as_ptr()) });
}