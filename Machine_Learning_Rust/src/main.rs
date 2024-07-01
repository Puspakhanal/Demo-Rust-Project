use std::error::Error;
use std::time::Instant;
use ndarray::{Array, Array2, Axis};
use linfa::Dataset;
use linfa::prelude::{Fit, Predict};
use linfa_trees::{DecisionTree, SplitQuality};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Listing {
    #[serde(rename = "Price")]
    price: f64,
}
// Load data from CSV into Vec<Listing>
fn load_data_from_csv(file_path: &str) -> Result<Vec<Listing>, Box<dyn Error>> {
    let mut csv_data = Vec::new();
    let mut rdr = csv::Reader::from_path(file_path)?;
    for result in rdr.deserialize() {
        let record: Listing = result?;
        csv_data.push(record);
    }
    Ok(csv_data)
}
// Extract Price into a 2D array
fn get_records(data: &[Listing]) -> Array2<f64> {
    let records = data.iter().map(|listing| listing.price).collect::<Vec<_>>();
    Array::from(records).insert_axis(Axis(1))
}
fn get_labels(data: &[Listing]) -> Vec<usize> {
    data.iter().map(|listing| {
        if listing.price < 10000.0 {
            0
        } else if listing.price < 50000.0 {
            1
        } else {
            2
        }
    }).collect()
}
fn create_dataset(file_path: &str) -> Result<Dataset<f64, usize, ndarray::Dim<[usize; 1]>>, Box<dyn Error>> {
    let data = load_data_from_csv(file_path)?;
    let records = get_records(&data);
    let labels = get_labels(&data);
    
    let dataset = Dataset::new(records, labels.into());
    Ok(dataset)
}

//  Main function
fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    let dataset = create_dataset(r"C:\Users\puspa\OneDrive\Desktop\Craiglist_listing.csv")?;
    let model = DecisionTree::params()
        .split_quality(SplitQuality::Gini)
        .max_depth(Some(100))
        .min_weight_split(2.0)
        .min_weight_leaf(1.0)
        .fit(&dataset)?;

    // Predict on training data
    let predictions = model.predict(&dataset);
    let actual = dataset.targets().to_vec();

    // Print predictions and actual targets
    println!("Predictions: {:?}", predictions);
    println!("Actual targets: {:?}", actual);

    // End time
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);

    Ok(())
}
