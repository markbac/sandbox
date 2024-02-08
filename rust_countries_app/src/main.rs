use serde_json::{self, Value};
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://restcountries.com/v3.1/all";
    let country_data = fetch_country_data(url).await?;
    let file_path = "countries.json";
    save_data_to_file(&country_data, file_path)?;
    Ok(())
}

async fn fetch_country_data(url: &str) -> Result<Value, reqwest::Error> {
    let response = reqwest::get(url).await?.json::<Value>().await?;
    Ok(response)
}

fn save_data_to_file(data: &Value, file_path: &str) -> std::io::Result<()> {
    let mut file = std::fs::File::create(file_path)?;
    let data_str = serde_json::to_string_pretty(data)?;
    file.write_all(data_str.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json; // Ensure json! macro is imported for the test
    use std::fs::{self, File};
    use std::io::Read;

    #[tokio::test]
    async fn test_fetch_country_data() {
        let url = "https://restcountries.com/v3.1/all";
        let result = fetch_country_data(url).await;
        assert!(result.is_ok(), "Failed to fetch country data");
    }

    #[test]
    fn test_save_data_to_file() {
        let data = json!({"test": "data"}); // This requires `serde_json::json`
        let file_path = "test_output.json";
        let save_result = save_data_to_file(&data, file_path);
        assert!(save_result.is_ok(), "Failed to save data to file");

        // Verify file contents
        let mut file = File::open(file_path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        assert_eq!(contents, serde_json::to_string_pretty(&data).unwrap());

        // Cleanup
        fs::remove_file(file_path).unwrap();
    }

    #[tokio::test]
    async fn integration_test_country_data_flow() {
        let url = "https://restcountries.com/v3.1/all";
        let country_data = fetch_country_data(url).await.unwrap();
        let file_path = "integration_test_countries.json";
        save_data_to_file(&country_data, file_path).unwrap();

        // Verify the file exists and is not empty
        let metadata = fs::metadata(file_path).unwrap();
        assert!(metadata.len() > 0, "File should not be empty");

        // Cleanup
        fs::remove_file(file_path).unwrap();
    }
}
