use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::Write;
use std::time::SystemTime;

#[derive(Parser)]
struct Cli {
    note_name: String,
}

const TEMPLATE_STRING: &str = "Tags:

---



---

# References
1. 
";

fn main() -> Result<()> {
    let args = Cli::parse();

    let unix_timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs();

    let mut file = File::create(format!("{} {}.md", unix_timestamp, &args.note_name))?;
    file.write_all(TEMPLATE_STRING.as_bytes())?;

    Ok(())
}
