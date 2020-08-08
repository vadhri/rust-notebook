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

##### Release (16MB json - 57,265 records)
- count_test_simple_nested_json: 147.219µs seconds for counting 1 records
- count_test_simple_with_filter_json: 164.697µs seconds.
- count_test_simple_json: 151.941µs seconds for counting 1 records
- count_test_simple_nested_with_filter_json: 339.415µs seconds
- filter_out_json_no_results: 631.994µs seconds.
- filter_out_json_by_value: 888.325µs seconds.
- count_airports: 20.457554ms seconds.
- filter_out_airports_no_results: 347.208947ms seconds seconds for filtering 57265 records
- count_with_filter_airports: 349.70095ms seconds seconds for filtering 57265 records
- filter_out_airports_in_country: 354.040548ms seconds for filtering 57265 records

##### Release (1.1 GB - appended 16MB json multiple times - 4,065,815 records)
- count_airports ... count_airports: 1.079399532s seconds.
- count_test_simple_json ... count_test_simple_json: 108.995µs seconds for counting 1 records
- count_test_simple_nested_json ... count_test_simple_nested_json: 73.032µs seconds for counting 1 records
- count_test_simple_nested_with_filter_json ... count_test_simple_nested_with_filter_json: 96.173µs seconds
- count_test_simple_with_filter_json ... count_test_simple_with_filter_json: 70.349µs seconds.
- count_with_filter_airports ... count_with_filter_airports: 20.170824135s seconds.
- filter_out_airports_in_country ... filter_out_airports_in_country 20.252878728s: seconds for filtering 57265 records
- filter_out_airports_no_results ... filter_out_airports_no_results: 20.181562593s seconds.
- filter_out_json_by_value ... filter_out_json_by_value: 547.087µs seconds.
- filter_out_json_no_results ... filter_out_json_no_results: 490.056µs seconds.
- write_distinct_fields ... write_distinct_fields: 460.137µs seconds.
- write_distinct_fields_large_json ... write_distinct_fields: 26.096326613s seconds.
