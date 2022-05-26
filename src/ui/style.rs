use console::Style;
use indicatif::ProgressStyle;

lazy_static! {
    pub static ref WHITE: Style = Style::new().white();

    pub static ref TICK_CHARS_BRAILLE_4_6_DOWN: String = String::from("⠶⢲⣰⣤⣆⡖");
    pub static ref TICK_CHARS_BRAILLE_4_6_UP: String = String::from("⠛⠹⠼⠶⠧⠏");
    pub static ref BRAILLE_6: String = String::from("⠿");
    
    pub static ref THIN_PROGRESS: String = String::from("━>-");
    pub static ref THIN_DUAL_PROGRESS: String = String::from("=>-");

    // pub static ref SPINNER: ProgressStyle = ProgressStyle::default_spinner()
    //     .template("{prefix:.bold.dim} {spinner.green} {wide_msg}")
    //     .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");
}


// pub fn spinner() ->  {
    
// }
