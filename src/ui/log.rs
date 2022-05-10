use console::Term;

use anyhow::Result;

pub fn info(msg: String) -> Result<()> {
    let term = Term::stdout();
    term.write_line(&msg)?;
    // term.clear_line()?;
    Ok(())
}
