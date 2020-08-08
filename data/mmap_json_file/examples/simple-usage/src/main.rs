use mmap_json_file;
use serde;
use serde::{Deserialize, Serialize};
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

fn main() {
    let start = Instant::now();
    let _res = mmap_json_file::count::<TestSimple>("data/test_simple.json".to_string());
    let end = Instant::now();

    println!("count_test_simple_json: {:?} seconds for counting 1 records", end - start);

    assert_eq!(_res.unwrap(), 1);
}
