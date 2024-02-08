use druid::widget::{Button, Flex, Label, TextBox, CrossAxisAlignment};
use druid::{AppLauncher, Data, Lens, LocalizedString, Widget, WidgetExt, WindowDesc, Env};
use rfd::FileDialog;
use std::{fs::File, io::{BufWriter, Write}, path::Path};
use calamine::{open_workbook, Reader, Xlsx};
use csv::Reader as CsvReader;
use serde_json::json;

#[derive(Clone, Data, Lens)]
struct AppState {
    input_path: String,
    output_path: String,
    message: String,
}

fn main() {
    let main_window = WindowDesc::new(|| build_ui())
        .title(LocalizedString::new("file-converter").with_placeholder("File Converter"));
    let initial_state = AppState {
        input_path: "".into(),
        output_path: "".into(),
        message: "Enter paths and press convert".into(),
    };
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_ui() -> impl Widget<AppState> {
    let input_path = TextBox::new()
        .with_placeholder("Input file path")
        .lens(AppState::input_path);
    let input_button = Button::new("Select Input File").on_click(|_ctx, data: &mut AppState, _env| {
        if let Some(path) = FileDialog::new().pick_file() {
            data.input_path = path.to_string_lossy().into_owned();
        }
    });

    let output_path = TextBox::new()
        .with_placeholder("Output JSON file path")
        .lens(AppState::output_path);
    let output_button = Button::new("Select Output File").on_click(|_ctx, data: &mut AppState, _env| {
        if let Some(path) = FileDialog::new().save_file() {
            data.output_path = path.to_string_lossy().into_owned();
        }
    });

    let convert_button = Button::new("Convert").on_click(|_ctx, data: &mut AppState, _env| {
        match convert(&data.input_path, &data.output_path) {
            Ok(_) => data.message = "Conversion Successful!".into(),
            Err(e) => data.message = format!("Error: {}", e),
        }
    });

    let message_label = Label::new(|data: &AppState, _env: &_| data.message.clone());

    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Flex::row().with_child(input_path).with_child(input_button))
        .with_child(Flex::row().with_child(output_path).with_child(output_button))
        .with_child(convert_button)
        .with_child(message_label)
}

fn convert(input_path: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let ext = Path::new(input_path)
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap_or("");

    match ext.to_lowercase().as_str() {
        "csv" => convert_csv(input_path, output_path)?,
        // "xlsx" => convert_xlsx(input_path, output_path)?,
        _ => return Err("Unsupported file format".into()),
    }

    Ok(())
}

fn convert_csv(input_path: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut rdr = CsvReader::from_path(input_path)?;
    let headers = rdr.headers()?.clone();

    let file = File::create(output_path)?;
    let mut writer = BufWriter::new(file);
    let mut json_array = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let mut json_object = serde_json::Map::new();
        for (header, value) in headers.iter().zip(record.iter()) {
            json_object.insert(header.to_string(), json!(value));
        }
        json_array.push(json_object);
    }

    write!(writer, "{}", serde_json::to_string(&json_array)?)?;
    Ok(())
}


/* 
fn convert_xlsx(input_path: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut excel: Xlsx<_> = open_workbook(input_path)?;
    let sheet_names = excel.sheet_names().to_owned();
    let sheet_name = sheet_names
        .first()
        .ok_or("No sheets found in XLSX file")?
        .to_string();

    // Correctly handle the Result returned by worksheet_range
    let range = excel.worksheet_range(&sheet_name)
        .map_err(|e| format!("Failed to read sheet range: {}", e).into())?;

    let mut iter = range.rows();
    let headers = iter.next().ok_or("No headers found")?;
    let mut json_array = Vec::new();

    for row in iter {
        let mut json_object = serde_json::Map::new();
        for (header, cell) in headers.iter().zip(row) {
            json_object.insert(header.to_string(), json!(cell.to_string()));
        }
        json_array.push(json_object);
    }

    let file = File::create(output_path)?;
    let mut writer = BufWriter::new(file);
    write!(writer, "{}", serde_json::to_string(&json_array)?)?;

    Ok(())
}
*/
