#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn handle_panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn process_transaction(amount: i32) -> i32 {
    amount + 10 // simple operation
}
