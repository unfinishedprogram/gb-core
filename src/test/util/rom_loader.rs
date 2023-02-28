use std::fs::read;

use crate::{test::boot::cgb_test_instance, Gameboy};

pub fn init_emulator_with_rom_cgb(src: &str) -> Gameboy {
    let mut state = cgb_test_instance();
    let rom = read(src).expect(src);
    state.load_rom(&rom, None);

    state
}
