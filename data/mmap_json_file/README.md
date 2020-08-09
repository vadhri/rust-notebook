## Rust and mmap
The library is published on crates.io as [mmap_json_file](https://crates.io/crates/mmap_json_file) and can help with counting and filtering json files with records all of which as symmetric in their structure ( json array of objects ) as the format below.

```[{..}, {..}, ..]```

The idea of using memory mapped i/o is check the performance while filtering and creating another file etc from rust while putting serde in harmsway :grin:.

Here is the best [read](http://lkml.iu.edu/hypermail/linux/kernel/0802.0/1496.html) on the topic from Linux forums.


### Functionality
- Count the number of records in JSON file.
- Count the number of records with filter
- Filter the JSON file with a condition ( provided by the caller ) and save it to a file specified.
- Distinct values of a key.

### Test
The airports JSON has been taken from the [location](https://datahub.io/core/airport-codes). It needs to be downloaded and put in the data/ directory.

Thanks to the original mmap [lib](https://crates.io/crates/memmap).

#### General performance seems to be of the order below.
( all tests below are run in a macbook )

```
cargo test --release  -- --nocapture --test-threads 1
```

##### Debug
- count_test_simple_nested_json: 256.37µs seconds for counting 1 records
- count_test_simple_with_filter_json: 322.471µs seconds.
- count_test_simple_nested_with_filter_json: 396.664µs seconds
- count_test_simple_json: 258.594µs seconds for counting 1 records
- filter_out_json_no_results: 675.752µs seconds.
- filter_out_json_by_value: 785.313µs seconds.
- count_airports: 690.802302ms seconds.
- count_with_filter_airports: 3.913697422s seconds for filtering 57265 records
- filter_out_airports_in_country 3.91415908s: seconds for filtering 57265 records
- filter_out_airports_no_results: 3.922528546s seconds for filtering 57265 records

##### Release (1.1 GB - appended 16MB json multiple times - 3,355,711 records)
- count_airports ... count_airports: 1.153728577s seconds.
- count_test_simple_json ... count_test_simple_json: 105.415µs seconds for counting 1 records
- count_test_simple_nested_json ... count_test_simple_nested_json: 137.288µs seconds for counting 1 records
- count_test_simple_nested_with_filter_json ... count_test_simple_nested_with_filter_json: 156.865µs seconds
- count_test_simple_with_filter_json ... count_test_simple_with_filter_json: 85.541µs seconds.
- filter_out_json_by_value ... filter_out_json_by_value: 697.84µs seconds.
- filter_out_json_no_results ... filter_out_json_no_results: 380.902µs seconds.
- write_distinct_fields ... write_distinct_fields: 576.174µs seconds.
- test_sum_over_field ... test_sum_over_field: 104.42µs seconds.
- count_with_filter_airports ... count_with_filter_airports: 17.461620452s seconds.
- filter_out_airports_in_country ... filter_out_airports_in_country 17.580610223s: seconds.
- filter_out_airports_no_results ... filter_out_airports_no_results: 17.333596128s seconds.
- test_sum_over_field_airport_elevation_ft ... test_sum_over_field: 17.291048913s seconds.
- write_distinct_fields_large_json ... write_distinct_fields: 22.316755059s seconds.

test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

##### Doc-tests mmap_json_file

running 5 tests
test src/lib.rs - count (line 294) ... ok
test src/lib.rs - count_with_filter (line 197) ... ok
test src/lib.rs - distinct_of_field (line 371) ... ok
test src/lib.rs - filter (line 47) ... ok
test src/lib.rs - sum_over_field (line 508) ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
