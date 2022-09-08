use console::Style;

use once_cell::sync::Lazy;

pub static WHITE: Lazy<Style> = Lazy::new(|| Style::new().white());

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


// pub fn spinner() ->  {
    
// }
