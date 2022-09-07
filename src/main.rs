use std::error::Error;
use std::io;

use grub_split_library::deserialize::{Deserialize, Ptr};
use grub_split_library::memory::external::ExternalMemoryReader;
use grub_split_library::memory::Address;

fn invalid_input(desc: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, desc)
}

fn usage(args: &[String]) -> io::Result<()> {
    if args.len() >= 1 {
        eprintln!("Usage: {} <pid> <addr>", args[0]);
    } else {
        eprintln!("Usage: <executable> <pid> <addr>");
    }
    Err(invalid_input("bad args"))
}

#[derive(Deserialize)]
struct ExternalData {
    x: usize,
    y: i32,
    z: i16,
    a: Ptr<u8>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        usage(args.as_slice())?;
    }

    let pid: i32 = args[1].parse()?;
    let mut reader = ExternalMemoryReader::from_pid(pid)?;
    let addr = Address::new(usize::from_str_radix(
        args[2]
            .strip_prefix("0x")
            .ok_or_else(|| invalid_input("expected hexadecimal address"))?,
        16,
    )?);

    println!("Reading pointer at address {} from {}", addr, pid);
    let data = ExternalData::deserialize(&mut reader, addr)?;
    println!("x is {}", data.x);
    println!("y is {}", data.y);
    println!("z is {}", data.z);
    println!("a is {:?}", data.a);
    println!("*a is {}", data.a.deref(&mut reader)?);
    Ok(())
}
