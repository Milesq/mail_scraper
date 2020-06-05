use csv::{Reader, Writer};
use std::{fs::File, io::Write, path::Path};

fn log(s: &'static str) -> ! {
    File::create("error.log")
        .unwrap()
        .write_all(s.as_bytes())
        .unwrap();

    panic!(s);
}

pub fn add_record(code: &str, file_output: &str, fields: &[String]) -> csv::Result<()> {
    if !Path::new(file_output).exists() {
        let mut writer = Writer::from_path(file_output)?;
        writer.write_record(fields)?;
        writer.flush()?;
    }

    let existing_data = get_existing(file_output).expect("Nieprawidłowy zapis danych w pliku");
    let data = parse_mail::parse(code.to_string()).expect("Nieprawidłowy zapis danych");
    let fields_data: Vec<_> = fields
        .iter()
        .map(|key| {
            data.get(key)
                .unwrap_or_else(|| log("Brak wskazanego pola w mailu"))
        })
        .collect();

    let mut writer = Writer::from_path(file_output)?;
    writer.write_record(fields)?;
    for data in existing_data {
        writer.write_record(&data)?;
    }
    writer.write_record(fields_data)?;
    writer.flush()?;

    Ok(())
}

fn get_existing(path: &str) -> csv::Result<Vec<csv::StringRecord>> {
    Ok(if let Ok(mut reader) = Reader::from_path(path) {
        reader
            .records()
            .map(|el| el.expect("Nieprawidłowy zapis danych"))
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    })
}
