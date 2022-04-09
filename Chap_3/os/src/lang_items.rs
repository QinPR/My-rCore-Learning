use core::panic::PanicInfo;
use crate::sbi::shutdown;


// 这个文件主要是实现panic这个宏的

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println! {
            "Panicked ar {}: {} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        };
    } else{
        println!("Panicked: {}", info.message().unwrap());
    }
    shutdown()
}