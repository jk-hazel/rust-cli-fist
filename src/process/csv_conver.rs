use serde_json::Value;

use crate::cli::OutputFormat;

pub fn process_csv(input: &str, output:String,format: OutputFormat) -> anyhow::Result<(),anyhow::Error> {
    let mut reader = csv::Reader::from_path(input)?;
    let mut json_vec = Vec::with_capacity(128);
    //获得csv文件的header
    let headers = reader.headers()?.clone();
    println!("headers: {:?}",headers);
    for record in reader.records() {
        let json_value = headers.iter().zip(record?.iter()).collect::<Value>();
        json_vec.push(json_value);
    }
    println!("input object: {:?}",json_vec);
    let content = match format {
        OutputFormat::JSON => serde_json::to_string_pretty(&json_vec)?,
        OutputFormat::TOML => toml::to_string_pretty(&json_vec)?,
        OutputFormat::YAML => serde_yaml::to_string(&json_vec)?,  
    };

    std::fs::write(output, content)?;
    Ok(())
}
