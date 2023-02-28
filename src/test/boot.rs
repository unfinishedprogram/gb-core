use crate::{lcd::GameboyLCD, Gameboy};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref BOOTED_EMULATOR: Gameboy = {
        let mut state = Gameboy::default();
        // Not a specific rom, just one that has a valid logo and will pass checks
        // TODO: Make this a custom rom that minimally satisfies the boot requirements
        let rom = *include_bytes!("../roms/test/dmg-acid2.gb");
        let lcd = GameboyLCD::default();

        state.bind_lcd(lcd);
        state.load_rom(&rom, None);

        state.run_until_boot();
        state
    };
}
