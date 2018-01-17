use std::fs::OpenOptions;
use std::io::Write;

#[no_mangle]
pub fn print_and_change_value_mac_os(x : *mut isize) -> isize {
    unsafe {
        println!("argument current value is {}", *x);
        let maybe_path = std::env::home_dir();
        let return_code = match maybe_path {
            Some(mut path) => { 
                path.push("result.txt");
                let f = OpenOptions::new().create(true).append(true).open(path);
                match f {
                    Ok(mut file) =>  {
                        let msg = format!("argument current value is {}\n", *x);
                        let result = file.write(msg.as_bytes());
                        if result.is_ok() {
                            0
                        } else {
                            1
                        }
                    },
                    Err(error) => { 
                        let kind = error.kind();
                        match kind {
                            std::io::ErrorKind::PermissionDenied =>  { 2 },
                            std::io::ErrorKind::NotFound =>  { 3 },
                            std::io::ErrorKind::ConnectionRefused =>  { 4 },
                            std::io::ErrorKind::ConnectionReset =>  { 5 },
                            std::io::ErrorKind::ConnectionAborted =>  { 6 },
                            std::io::ErrorKind::NotConnected =>  { 7 },
                            std::io::ErrorKind::AddrInUse =>  { 8 },
                            std::io::ErrorKind::AddrNotAvailable =>  { 9 },
                            std::io::ErrorKind::BrokenPipe =>  { 10 },
                            std::io::ErrorKind::AlreadyExists =>  { 11 },
                            std::io::ErrorKind::WouldBlock =>  { 12 },
                            std::io::ErrorKind::InvalidInput =>  { 13 },
                            std::io::ErrorKind::InvalidData =>  { 14 },
                            std::io::ErrorKind::TimedOut =>  { 15 },
                            std::io::ErrorKind::WriteZero =>  { 16 },
                            std::io::ErrorKind::Interrupted =>  { 17 },
                            std::io::ErrorKind::Other =>  { 18 },
                            std::io::ErrorKind::UnexpectedEof =>  { 19 },
                            _ =>  { 20 }
                        }
                    }
                }
            },
            None => 99
        };

        let desired_value : isize = 42;
        println!("setting argument to {}", desired_value);
        *x = desired_value;

        return_code
    }
}

#[no_mangle]
pub extern fn print_and_change_value_ios(x : *mut isize)  {
    unsafe {
        println!("argument current value is {}", *x);
        let desired_value : isize = 42;
        println!("setting argument to {}", desired_value);
        *x = desired_value;
    }
}
