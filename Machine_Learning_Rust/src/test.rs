#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    // Helper function to create a temporary CSV file
    fn create_temp_csv(content: &str) -> String {
        let file_path = "temp_test_file.csv";
        let mut file = File::create(file_path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
        file_path.to_string()
    }

    #[test]
    fn test_load_data_from_csv() {
        let csv_content = "Price,Location\n100.0,LocationA\n200.0,LocationB\n";
        let file_path = create_temp_csv(csv_content);
        let data = load_data_from_csv(&file_path).expect("Failed to load data from CSV");
        
        assert_eq!(data.len(), 2);
        assert_eq!(data[0], Listing { price: 100.0, location: Some("LocationA".to_string()) });
        assert_eq!(data[1], Listing { price: 200.0, location: Some("LocationB".to_string()) });
    }

    #[test]
    fn test_transformed_data() {
        let csv_content = "Price,Location\n100.0,LocationA\n200.0,LocationB\n";
        let file_path = create_temp_csv(csv_content);
        let data = load_data_from_csv(&file_path).expect("Failed to load data from CSV");

        let mut location_map: HashMap<&str, f64> = HashMap::new();
        let mut code = 0.0;
        for listing in &data {
            if let Some(location) = &listing.location {
                if !location_map.contains_key(location.as_str()) {
                    location_map.insert(location.as_str(), code);
                    code += 1.0;
                }
            }
        }

        let transformed_data: Vec<(f64, f64)> = data.iter()
            .filter_map(|listing| {
                listing.location.as_ref().map(|loc| (*location_map.get(loc.as_str()).unwrap_or(&0.0), listing.price))
            })
            .collect();

        assert_eq!(transformed_data.len(), 2);
        assert_eq!(transformed_data[0], (0.0, 100.0));
        assert_eq!(transformed_data[1], (1.0, 200.0));
    }

    #[test]
    fn test_array2_conversion() {
        let transformed_data = vec![(0.0, 100.0), (1.0, 200.0)];
        let num_samples = transformed_data.len();
        let x: Array2<f64> = Array2::from_shape_vec((num_samples, 2), transformed_data.iter()
            .flat_map(|&(loc, price)| vec![loc, price])
            .collect())
            .expect("Failed to create Array2<f64>");

        assert_eq!(x.shape(), &[2, 2]);
        assert_eq!(x[[0, 0]], 0.0);
        assert_eq!(x[[0, 1]], 100.0);
        assert_eq!(x[[1, 0]], 1.0);
        assert_eq!(x[[1, 1]], 200.0);
    }
}
