# Polars DataFrames

## Running the code

Run the following command in your terminal:

```bash
cargo run
```

There are some command options you can use, such as:

```bash
# To sort column values
cargo run -- sort

# To print the `.csv` file
cargo run -- print

#  shape of the `.csv` file
cargo run -- shape

# Show schema of the `.csv` file
cargo run -- schema

# Describe the 
cargo run -- describe
```

For more information and details about parameters, run: 

```
cargo run -- --help
```

## Improvements

- Created the `sort-by` parameter for the `sort` command, so that the results can be sorted by any given column;

- Added a new CLI command, `filter`, to filter rows based on a given condition. For example, filter countries with life expectancy greater than 70 in the year 2020.