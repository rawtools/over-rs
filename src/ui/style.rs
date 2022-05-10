use console::Style;
use indicatif::ProgressStyle;

lazy_static! {
    pub static ref WHITE: Style = Style::new().white();

    pub static ref SPINNER: ProgressStyle = ProgressStyle::default_spinner()
        .template("{prefix:.bold.dim} {spinner} {wide_msg}")
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");
}


// pub fn spinner() ->  {
    
// }
