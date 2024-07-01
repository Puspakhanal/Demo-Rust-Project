use std::error::Error;
use std::sync::Arc;
use neo4rs::*;
use reqwest;
use scraper::{Html, Selector};
use tokio;
/// Main function to scrape real estate listings from Craigslist and store them in Neo4j
#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn Error>> {
    // Neo4j connection details
    let uri = "neo4j+s://8e85c770.databases.neo4j.io";
    let user = "neo4j";
    let pass = "J_0vmUTd1OLmLQTsV0Anc76g7tODvXihGzf_vnkSqCw";
    // Connect to Neo4j
    let graph = Arc::new(Graph::new(uri, user, pass).await?);
    // List of Craigslist real estate listing URLs for different locations
    let urls = vec![
        "https://newyork.craigslist.org/search/rea",
        "https://newjersey.craigslist.org/search/rea",
        "https://austin.craigslist.org/search/rea", // Texas - Austin
        "https://losangeles.craigslist.org/search/rea",
        "https://sfbay.craigslist.org/search/rea" // California - Bay Area
    ];
    // Loop through each URL
    for url in urls {
        println!("Fetching URL: {}", url);
        // Make an HTTP GET request to fetch the page content
        let response = reqwest::get(url).await?.text().await?;
        println!("Fetched content length: {}", response.len());
        // Check if the response contains expected HTML
        if response.contains("<html") {
            println!("Response contains HTML content.");
        } else {
            println!("Response does not contain expected HTML content.");
            continue;
        }
        // Parse the HTML content
        let document = Html::parse_document(&response);
        // Define the CSS selectors for the data you want to extract
        let listing_selector = Selector::parse("li.cl-static-search-result").unwrap();
        let title_selector = Selector::parse("div.title").unwrap();
        let price_selector = Selector::parse("div.price").unwrap();
        let location_selector = Selector::parse("div.location").unwrap();
        let link_selector = Selector::parse("a").unwrap();
        // Check if any listings are found
        let listings: Vec<_> = document.select(&listing_selector).collect();
        println!("Found {} listings.", listings.len());
        // Extract and print the data
        for listing in &listings {
            let title = listing.select(&title_selector).next().map_or_else(|| "N/A".to_string(), |e| e.text().collect::<String>());
            let price = listing.select(&price_selector).next().map_or_else(|| "N/A".to_string(), |e| e.text().collect::<String>());
            let location = listing.select(&location_selector).next().map_or_else(|| "N/A".to_string(), |e| e.text().collect::<String>());
            let detail_link = listing.select(&link_selector).next().map_or_else(|| "N/A".to_string(), |e| e.value().attr("href").unwrap_or("N/A").to_string());
            println!("Title: {}, Price: {}, Location: {}, Detail Link: {}", title, price, location, detail_link);
            // Insert data into Neo4j
            let query = query("CREATE (listing:Listing {title: $title, price: $price, location: $location, detail_link: $detail_link})")
                .param("title", title)
                .param("price", price)
                .param("location", location)
                .param("detail_link", detail_link);
            match graph.run(query).await {
                Ok(_) => println!("Data inserted successfully."),
                Err(e) => println!("Failed to insert data: {:?}", e),
            }
        }
        if listings.is_empty() {
            println!("No listings found for URL: {}. Please check the CSS selectors and the webpage content.", url);
        }
    }
    Ok(())
}
