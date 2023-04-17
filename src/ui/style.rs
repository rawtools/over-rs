use console::{style, StyledObject};

use once_cell::sync::Lazy;

pub static TICK_CHARS_BRAILLE_4_6_DOWN: Lazy<String> = Lazy::new(|| String::from("⠶⢲⣰⣤⣆⡖"));
pub static TICK_CHARS_BRAILLE_4_6_UP: Lazy<String> = Lazy::new(|| String::from("⠛⠹⠼⠶⠧⠏"));
pub static BRAILLE_6: Lazy<String> = Lazy::new(|| String::from("⠿"));

pub static THIN_PROGRESS: Lazy<String> = Lazy::new(|| String::from("━>-"));
pub static THIN_DUAL_PROGRESS: Lazy<String> = Lazy::new(|| String::from("=>-"));

pub static DOTS_4: Lazy<String> = Lazy::new(|| String::from("::"));
    
    // pub static ref SPINNER: ProgressStyle = ProgressStyle::default_spinner()
    //     .template("{prefix:.bold.dim} {spinner.green} {wide_msg}")
    //     .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");
// }


pub fn white<D>(value: D) -> StyledObject<D> {
    style(value).white()
}

pub fn white_b<D>(value: D) -> StyledObject<D> {
    white(value).bold()
}

pub fn white_bi<D>(value: D) -> StyledObject<D> {
    white_b(value).italic()
}
