extern crate arrow;

use std::fs::File;
use std::sync::Arc;

use arrow::csv;
use arrow::datatypes::{DataType, Field, Schema};
use arrow::error::Result;
use arrow::util::pretty;
use arrow::util::pretty::print_batches;

use datafusion::datasource::csv::CsvReadOptions;
use datafusion::execution::context::ExecutionContext;

fn main() -> Result<()> {
    let file = File::open("data/WEOApr2020all.csv").unwrap();

    // Datafusion - read csv file and execute SQL commands.

    let mut ctx = ExecutionContext::new();

    ctx.register_csv(
        "weo_april_2020",
        "data/WEOApr2020all.csv",
        CsvReadOptions::new(),
    );

    let sql = "SELECT ISO,n1980,n1981,n1982,n1983 FROM weo_april_2020";

    let plan = ctx.create_logical_plan(sql).unwrap();
    let plan = ctx.optimize(&plan).unwrap();
    let plan = ctx.create_physical_plan(&plan, 1024 * 1024).unwrap();

    let results = ctx.collect(plan.as_ref()).unwrap();

    pretty::print_batches(&results);

    // Read CSV file and load column format into memory.

    let builder = csv::ReaderBuilder::new()
        .has_header(true)
        .infer_schema(Some(100));
    let mut csv = builder.build(file).unwrap();

    let mut total_no_of_rows = 0;

    while let Some(_batch) = csv.next().unwrap() {
        total_no_of_rows += _batch.num_rows();
    }

    println!("Total rows = {:?}", total_no_of_rows);

    Ok(())
}
