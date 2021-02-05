// DataFrame manipulation using Polars!
// This crate provides similar functionalities
// as the Pandas library in Python. There is
// also a Python API of Polars providing the
// benefits (aka speed) of Polars all witin
// Python.
//

// This is still work in progress. 
// 
// TO DO!!:
// Add computation functionalites on dataframe (selection, indexing, joins, iterators, grouping, pivot tables)
// Break down some of the main function sections in different functions for better readability
// Create a utils script for using when conducting an analysis using Polars in Rust


use polars::prelude::* 

fn main()  {
    // 1) Create a DataFrame with data
    // A DataFrame is created from a Vec of Series.
    let dates = &[
        "2021-02-09",
        "2021-02-04",
        "2021-02-08",
        "2021-02-06",
        "2021-02-07",
    ];
    let fmt = "%Y-%m-%d";
    let s0 = Date32Chunked::parse_from_str_slice("dates", dates, fmt).into();
    let s1 = Series::new("n", &[1, 2, 3, 4, 5]);
    let s2 = Utf8Chunked::full("channel", "email", 5).into();

    let df = DataFrame::new(vec![s0, s1, s2]).expect("something went wrong");

    // Sort the dataframe by date 
    let reverse = false;
    df.sort("dates", reverse).expect("column not sortable")

    println!("{}", df) ;

    // Get the columns data types of the resulting DataFrame
        df.dtypes()
        .iter()
        .zip(df.get_column_names().iter())
        .for_each(|(dtype, name)| 
        println!("Column: '{}',\t dtype: {:?}", name, dtype))

}


