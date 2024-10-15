#![no_std]
#![no_main]

#[no_mangle]
pub extern "C" fn main() {
    game::entry();
}

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop { }
}