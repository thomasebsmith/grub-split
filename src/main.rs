use std::error::Error;
use std::io;

use crate::memory::{Address, AddressRange, MemoryReader};
use crate::memory::external::ExternalMemoryReader;

pub mod memory;

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

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        usage(args.as_slice())?;
    }

    let pid: i32 = args[1].parse()?;
    let mut reader = ExternalMemoryReader::from_pid(pid)?;
    let addr: usize = usize::from_str_radix(
        args[2].strip_prefix("0x")
            .ok_or_else(|| invalid_input("expected hexadecimal address"))?,
        16
    )?;

    println!("Reading 8 bytes at address {} from {}", addr, pid);
    let result = reader.read(
        AddressRange{start: Address::new(addr), num_bytes: 8}
    )?;
    let hex = result.iter()
        .map(|byte| format!("{:02X}", byte))
        .collect::<String>();
    println!("{}", hex);
    Ok(())
}
