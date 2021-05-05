use std::fs::File;
use std::io::BufReader;
use std::env;
use prettytable::{Table,Row,Cell,Attr,format};
use exif::Reader;

#[macro_use] extern crate prettytable;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_BOX_CHARS);
    let args: Vec<String> = env::args().collect();

    // Add title before content
    let tag = Cell::new_align("Tag", format::Alignment::CENTER).with_style(Attr::Bold);
    let value = Cell::new_align("Value", format::Alignment::CENTER).with_style(Attr::Bold);
    let title = Row::new(vec![tag, value]);
    table.add_row(title);

    // Iterate over fields
    for path in &[&args[1]] {
        let file = File::open(path)?;
        let mut bufreader = BufReader::new(&file);
        let exifreader = Reader::new();
        let exif = exifreader.read_from_container(&mut bufreader)?;
        for f in exif.fields() {
            let tag = f.tag.description();
            let value = f.display_value().with_unit(&exif).to_string();
            if let Some(t) = tag {
                if value.len() < 100 {
                    table.add_row(row![t, value]);
                } else {
                    let v = format!("{:.*}...", 50, value);
                    table.add_row(row![t, v]);
                }
            } else {
                break
            }
        }
    }

    table.printstd();
    Ok(())
}
