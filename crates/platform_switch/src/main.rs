#![no_std]
#![no_main]

#[no_mangle]
pub fn main() -> nx::result::Result<()> {
    game::entry();

    Ok(())
}

#[no_mangle]
pub fn initialize_heap(hbl_heap: nx::util::PointerAndSize) -> nx::util::PointerAndSize {
    if hbl_heap.is_valid() {
        hbl_heap
    } else {
        let heap_size: usize = 0x10000000;
        let heap_address = nx::svc::set_heap_size(heap_size).unwrap();
        nx::util::PointerAndSize::new(heap_address, heap_size)
    }
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    nx::util::simple_panic_handler::<nx::diag::log::lm::LmLogger>(info, nx::diag::abort::AbortLevel::FatalThrow())
}