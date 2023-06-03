#![feature(fs_try_exists)]

mod haxxstation;
mod nds;

use std::fs::read;
use std::path::Path;

use ctru::prelude::*;
use ctru::services::romfs::RomFS;

fn main() {
    ctru::use_panic_handler();

    let apt = Apt::new().unwrap();
    let mut hid = Hid::new().unwrap();
    let gfx = Gfx::new().unwrap();
    let _console = Console::new(gfx.top_screen.borrow_mut());

    // init RomFS
    let _romfs = RomFS::new().unwrap();

    while apt.main_loop() {
        //Scan all the inputs. This should be done once for each frame
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::START) {
            break;
        }

        //Wait for VBlank
        gfx.wait_for_vblank();
    }
}
