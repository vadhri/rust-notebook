## Rust and mmap
The library is published in [crates.io](https://crates.io/crates/mmap_json_file) and can help with counting and filtering json files with symmetric records as the format below.

```[{..}, {..}, ..]```

The idea of using memory mapped i/o is check the performance while filtering and creating another file etc from rust while putting serde in harmsway :grin:. Although, here is the best [read](http://lkml.iu.edu/hypermail/linux/kernel/0802.0/1496.html) on the topic from Linux.

General performance seems to be of the order below.

count_test_simple_nested_json: 256.37µs seconds for counting 1 records
count_test_simple_with_filter_json: 322.471µs seconds.
count_test_simple_nested_with_filter_json: 396.664µs seconds
count_test_simple_json: 258.594µs seconds for counting 1 records
filter_out_json_no_results: 675.752µs seconds.
filter_out_json_by_value: 785.313µs seconds.
count_airports: 690.802302ms seconds.
count_with_filter_airports: 3.913697422s seconds for filtering 57265 records
filter_out_airports_in_country 3.91415908s: seconds for filtering 57265 records
filter_out_airports_no_results: 3.922528546s seconds for filtering 57265 records

### Functionality
- Count the number of records in JSON file.
- Count the number of records with filter
- Filter the JSON file with a condition ( provided by the caller ) and save it to a file specified.

### Test
The airports JSON has been taken from the [location](https://datahub.io/core/airport-codes). It needs to be downloaded and put in the data/ directory.

Thanks to the original mmap lib. 
