use csv::Writer;
use neo4rs::{Graph, query};
use regex::Regex;
use std::error::Error;
use tokio;
/// The main function to fetch data from Neo4j, clean it, and export it to a CSV file.

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn Error>> {
    // Neo4j connection details
    let uri = "neo4j+s://794f677f.databases.neo4j.io";
    let username = "neo4j";
    let password = "J_0vmUTd1OLmLQTsV0Anc76g7tODvXihGzf_vnkSqCw";

    // Connect to Neo4j
    let graph = Graph::new(uri, username, password).await?;

    // Cypher query
    let query = query("MATCH (listing:Listing) RETURN listing.title AS title, listing.price AS price, listing.location AS location, listing.detail_link AS detail_link");
    let mut result = graph.execute(query).await?;
    // Prepare data for CSV and defining CSV path
    let mut data = Vec::new();
    while let Ok(Some(row)) = result.next().await {
        let title: String = row.get("title").ok_or("Missing title")?;
        let price: String = row.get("price").ok_or("Missing price")?;
        let location: String = row.get("location").ok_or("Missing location")?;
        let detail_link: String = row.get("detail_link").ok_or("Missing detail_link")?;

        let cleaned_row = (
            clean_text(&title),
            clean_text(&price),
            clean_text(&location),
            clean_text(&detail_link),
        );
        data.push(cleaned_row);
    }
    let csv_file = r"C:\Users\puspa\OneDrive\Desktop\Craiglist_listing.csv";
    let mut wtr = Writer::from_path(csv_file)?;
    wtr.write_record(&["Title", "Price", "Location", "Detail Link"])?;
    for row in data {
        wtr.write_record(&[row.0, row.1, row.2, row.3])?;
    }
    wtr.flush()?;
    println!("Data exported to {}", csv_file);
    Ok(())
}

// Function to clean text data 
fn clean_text(text: &str) -> String {
    let patterns = vec![
        r"🏡", r"💛", r"!!!", r"\*\*\*\*\*", r"\*\*\*\*", r"\*\*\*", r"\*\$", r",", r"\*", r"!", r"\!\!", r"\.", r"\?", r"~~", r"~", r"\+", r"\$"
    ];
    let mut cleaned_text = text.to_string();
    for pattern in patterns {
        let re = Regex::new(pattern).unwrap();
        cleaned_text = re.replace_all(&cleaned_text, "").to_string();
    }
    cleaned_text.trim().to_string()
}
