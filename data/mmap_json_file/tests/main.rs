#[cfg(test)]
mod tests {
    use large_json_file;
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
        let _res = large_json_file::count::<TestSimple>(d.to_str().unwrap().to_string());
        let end = Instant::now();

        println!("{:?} seconds for counting 1 records", end - start);

        assert_eq!(_res.unwrap(), 1);
    }

    #[test]
    fn count_test_simple_nested_json() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("data/test_simple_nested.json");

        let start = Instant::now();
        let _res = large_json_file::count::<TestSimpleCompound>(d.to_str().unwrap().to_string());
        let end = Instant::now();

        println!("{:?} seconds for counting 1 records", end - start);

        assert_eq!(_res.unwrap(), 1);
    }
    #[test]
    fn count_test_simple_with_filter_json() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("data/test_simple.json");
        let filter = |record: TestSimple| -> bool { record.a.unwrap() == "b" };

        let start = Instant::now();
        let _res = large_json_file::count_with_filter::<TestSimple, Box<dyn Fn(TestSimple) -> bool>>(
            d.to_str().unwrap().to_string(),
            Box::new(filter),
        );
        let end = Instant::now();

        println!("{:?} seconds for counting 1 records", end - start);

        assert_eq!(_res.unwrap(), 1);
    }

    #[test]
    fn count_test_simple_nested_with_filter_json() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("data/test_simple_nested.json");
        let filter = |record: TestSimpleCompound| -> bool { record.f.unwrap() == "g" };

        let start = Instant::now();
        let _res = large_json_file::count_with_filter::<TestSimpleCompound, Box<dyn Fn(TestSimpleCompound) -> bool>>(
            d.to_str().unwrap().to_string(),
            Box::new(filter),
        );
        let end = Instant::now();

        println!("{:?} seconds for counting 1 records", end - start);

        assert_eq!(_res.unwrap(), 1);
    }


    #[test]
    fn filter_out_airports_in_country() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("data/airports.json");

        let filter = |record: AirportCodes| -> bool { record.iso_country.unwrap() == "IN" };
        let start = Instant::now();
        let _res = large_json_file::filter::<AirportCodes, Box<dyn Fn(AirportCodes) -> bool>>(
            d.to_str().unwrap().to_string(),
            Box::new(filter),
            "filtered_output.json".to_string(),
        );
        let end = Instant::now();

        println!("{:?} seconds for filtering 57265 records", end - start);
    }

    #[test]
    fn filter_out_json_by_value() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("data/test_simple.json");

        let filter = |record: TestSimple| -> bool { record.a.unwrap() == "b" };

        let _res = large_json_file::filter::<TestSimple, Box<dyn Fn(TestSimple) -> bool>>(
            d.to_str().unwrap().to_string(),
            Box::new(filter),
            "filtered_output_test_simple.json".to_string(),
        );
    }

    #[test]
    fn filter_out_json_no_results() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("data/test_simple.json");

        let filter = |record: TestSimple| -> bool { record.a.unwrap() == "Nothing" };

        let _res = large_json_file::filter::<TestSimple, Box<dyn Fn(TestSimple) -> bool>>(
            d.to_str().unwrap().to_string(),
            Box::new(filter),
            "filtered_output_test_simple_no_result.json".to_string(),
        );
    }

    #[test]
    fn filter_out_airports_no_results() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("data/airports.json");

        let filter = |record: AirportCodes| -> bool { record.iso_country.unwrap() == "NoCountry" };

        let _res = large_json_file::filter::<AirportCodes, Box<dyn Fn(AirportCodes) -> bool>>(
            d.to_str().unwrap().to_string(),
            Box::new(filter),
            "filtered_output_NoCountry.json".to_string(),
        );
    }

    #[test]
    fn count_with_filter_airports() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("data/airports.json");

        let filter = |record: AirportCodes| -> bool { record.iso_country.unwrap() == "IN" };

        let _res = large_json_file::count_with_filter::<
            AirportCodes,
            Box<dyn Fn(AirportCodes) -> bool>,
        >(d.to_str().unwrap().to_string(), Box::new(filter));

        assert_eq!(_res.unwrap(), 341);
    }

    #[test]
    fn count_airports() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("data/airports.json");

        let filter = |record: AirportCodes| -> bool { record.iso_country.unwrap() == "IN" };

        let start = Instant::now();
        let _res = large_json_file::count::<AirportCodes>(d.to_str().unwrap().to_string());
        let end = Instant::now();

        println!("{:?} seconds for counting 57265 records", end - start);

        assert_eq!(_res.unwrap(), 57265);
    }
}
