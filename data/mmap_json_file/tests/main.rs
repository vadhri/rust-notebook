#[cfg(test)]
mod tests {
    use mmap_json_file;
    use serde;
    use serde::{Deserialize, Serialize};
    use std::path::PathBuf;
    use std::str;
    use std::time::Instant;

    #[derive(Serialize, Deserialize, Debug, Clone, Default)]
    struct AirportCodes {
        continent: Option<String>,
        coordinates: Option<String>,
        elevation_ft: Option<String>,
        gps_code: Option<String>,
        iata_code: Option<String>,
        ident: Option<String>,
        iso_country: Option<String>,
        iso_region: Option<String>,
        local_code: Option<String>,
        municipality: Option<String>,
        name: Option<String>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, Default)]
    struct TestSimple {
        a: Option<String>,
        c: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, Default)]
    struct TestSimpleNested {
        b: Option<TestSimple>,
        c: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, Default)]
    struct TestSimpleCompound {
        a: Option<TestSimpleNested>,
        f: Option<String>,
    }

    #[test]
    fn count_test_simple_json() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("data/test_simple.json");

        let start = Instant::now();
        let _res = mmap_json_file::count::<TestSimple>(d.to_str().unwrap().to_string());
        let end = Instant::now();

        println!(
            "count_test_simple_json: {:?} seconds for counting 1 records",
            end - start
        );

        assert_eq!(_res.unwrap(), 1);
    }

    #[test]
    fn count_test_simple_nested_json() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("data/test_simple_nested.json");

        let start = Instant::now();
        let _res = mmap_json_file::count::<TestSimpleCompound>(d.to_str().unwrap().to_string());
        let end = Instant::now();

        println!(
            "count_test_simple_nested_json: {:?} seconds for counting 1 records",
            end - start
        );

        assert_eq!(_res.unwrap(), 1);
    }
    #[test]
    fn count_test_simple_with_filter_json() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("data/test_simple.json");
        let filter = |record: TestSimple| -> bool { record.a.unwrap() == "b" };

        let start = Instant::now();
        let _res = mmap_json_file::count_with_filter::<TestSimple, Box<dyn Fn(TestSimple) -> bool>>(
            d.to_str().unwrap().to_string(),
            Box::new(filter),
        );
        let end = Instant::now();

        println!(
            "count_test_simple_with_filter_json: {:?} seconds.",
            end - start
        );

        assert_eq!(_res.unwrap(), 1);
    }

    #[test]
    fn count_test_simple_nested_with_filter_json() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("data/test_simple_nested.json");
        let filter = |record: TestSimpleCompound| -> bool { record.f.unwrap() == "g" };

        let start = Instant::now();
        let _res = mmap_json_file::count_with_filter::<
            TestSimpleCompound,
            Box<dyn Fn(TestSimpleCompound) -> bool>,
        >(d.to_str().unwrap().to_string(), Box::new(filter));
        let end = Instant::now();

        println!(
            "count_test_simple_nested_with_filter_json: {:?} seconds",
            end - start
        );

        assert_eq!(_res.unwrap(), 1);
    }

    #[test]
    fn filter_out_airports_in_country() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("data/airports-dup.json");

        let filter = |record: AirportCodes| -> bool { record.iso_country.unwrap() == "IN" };
        let start = Instant::now();
        let _res = mmap_json_file::filter::<AirportCodes, Box<dyn Fn(AirportCodes) -> bool>>(
            d.to_str().unwrap().to_string(),
            Box::new(filter),
            "filtered_output.json".to_string(),
        );
        let end = Instant::now();

        println!(
            "filter_out_airports_in_country {:?}: seconds.",
            end - start
        );
    }

    #[test]
    fn filter_out_json_by_value() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("data/test_simple.json");

        let filter = |record: TestSimple| -> bool { record.a.unwrap() == "b" };
        let start = Instant::now();

        let _res = mmap_json_file::filter::<TestSimple, Box<dyn Fn(TestSimple) -> bool>>(
            d.to_str().unwrap().to_string(),
            Box::new(filter),
            "filtered_output_test_simple.json".to_string(),
        );
        let end = Instant::now();

        println!("filter_out_json_by_value: {:?} seconds.", end - start);
    }

    #[test]
    fn filter_out_json_no_results() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("data/test_simple.json");

        let filter = |record: TestSimple| -> bool { record.a.unwrap() == "Nothing" };
        let start = Instant::now();
        let _res = mmap_json_file::filter::<TestSimple, Box<dyn Fn(TestSimple) -> bool>>(
            d.to_str().unwrap().to_string(),
            Box::new(filter),
            "filtered_output_test_simple_no_result.json".to_string(),
        );
        let end = Instant::now();

        println!("filter_out_json_no_results: {:?} seconds.", end - start);
    }

    #[test]
    fn filter_out_airports_no_results() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("data/airports-dup.json");

        let filter = |record: AirportCodes| -> bool { record.iso_country.unwrap() == "NoCountry" };

        let start = Instant::now();
        let _res = mmap_json_file::filter::<AirportCodes, Box<dyn Fn(AirportCodes) -> bool>>(
            d.to_str().unwrap().to_string(),
            Box::new(filter),
            "filtered_output_NoCountry.json".to_string(),
        );
        let end = Instant::now();

        println!("filter_out_airports_no_results: {:?} seconds.", end - start);
    }

    #[test]
    fn count_with_filter_airports() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("data/airports-dup.json");

        let filter = |record: AirportCodes| -> bool { record.iso_country.unwrap() == "IN" };

        let start = Instant::now();
        let _res = mmap_json_file::count_with_filter::<
            AirportCodes,
            Box<dyn Fn(AirportCodes) -> bool>,
        >(d.to_str().unwrap().to_string(), Box::new(filter));
        let end = Instant::now();

        println!("count_with_filter_airports: {:?} seconds.", end - start);

        assert_eq!(_res.unwrap(), 19891);
    }

    #[test]
    fn count_airports() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("data/airports-dup.json");

        let filter = |record: AirportCodes| -> bool { record.iso_country.unwrap() == "IN" };

        let start = Instant::now();
        let _res = mmap_json_file::count::<AirportCodes>(d.to_str().unwrap().to_string());
        let end = Instant::now();

        println!("count_airports: {:?} seconds.", end - start);

        assert_eq!(_res.unwrap(), 3355711);
    }

    #[test]
    fn write_distinct_fields_large_json() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("data/airports-dup.json");

        let filter = |record: AirportCodes| -> String { record.iso_country.unwrap() };

        let start = Instant::now();
        let _res = mmap_json_file::distinct_of_field::<AirportCodes, Box<dyn Fn(AirportCodes) -> String>>(
            d.to_str().unwrap().to_string(),
            Box::new(filter),
            "distinct_test_simple.json".to_string(),
        );
        let end = Instant::now();

        println!("write_distinct_fields: {:?} seconds.", end - start);

        assert_eq!(_res.unwrap(), 243);
    }

    #[test]
    fn write_distinct_fields() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("data/test_simple.json");

        let filter = |record: TestSimple| -> String { record.a.unwrap() };

        let start = Instant::now();
        let _res = mmap_json_file::distinct_of_field::<TestSimple, Box<dyn Fn(TestSimple) -> String>>(
            d.to_str().unwrap().to_string(),
            Box::new(filter),
            "distinct_test_simple.json".to_string(),
        );
        let end = Instant::now();

        println!("write_distinct_fields: {:?} seconds.", end - start);

        assert_eq!(_res.unwrap(), 1);
    }

    #[test]
    fn test_sum_over_field() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("data/test_simple_sum.json");

        let filter = |record: TestSimple| -> f64 {
            match record.a {
                Some(value) => {
                    match value.parse::<f64>() {
                        Ok(num) => num as f64,
                        _ => 0f64
                    }
                },
                _ => 0f64
            }
        };

        let start = Instant::now();
        let _res = mmap_json_file::sum_over_field::<TestSimple, Box<dyn Fn(TestSimple) -> f64>, f64>(
            d.to_str().unwrap().to_string(),
            Box::new(filter)
        );
        let end = Instant::now();

        println!("test_sum_over_field: {:?} seconds.", end - start);

        assert_eq!(_res.unwrap(), 0.1200000000000001f64);
    }

    #[test]
    fn test_sum_over_field_airport_elevation_ft() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("data/airports-dup.json");

        let filter = |record: AirportCodes| -> f64 {
            match record.elevation_ft {
                Some(value) => {
                    match value.parse::<f64>() {
                        Ok(num) => num as f64,
                        _ => 0f64
                    }
                },
                _ => 0f64
            }
        };

        let start = Instant::now();
        let _res = mmap_json_file::sum_over_field::<AirportCodes, Box<dyn Fn(AirportCodes) -> f64>, f64>(
            d.to_str().unwrap().to_string(),
            Box::new(filter)
        );
        let end = Instant::now();

        println!("test_sum_over_field: {:?} seconds.", end - start);

        assert_eq!(_res.unwrap(), 3627266884.0f64);
    }
}
