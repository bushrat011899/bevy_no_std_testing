#![no_main]
#![no_std]

use uefi::prelude::*;

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    
    game::entry();

    uefi::boot::stall(10_000_000);
    Status::SUCCESS
}