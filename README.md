# extrack - extracting information out of your expenses

extrack is a CLI written in Rust to easily parse and summarize bank transaction excel sheets.
It can be used to find information related to your spending and earnings in total and by category, 
straight from your command line without having to send any data to a server.

## Installation
Installation requires Rust with Cargo installed.

```bash
git clone https://github.com/vifraa/extrack.git
cd extrack
cargo build --release
```


## Usage
The most basic usage is by entering the path to the excel sheet as an input argument.
For example: 
```bash
extrack <path_to_input_file>
```

This will output the result straight to the command line.
To save the result in a file you can either direct the result from the above command or give a path to where the output will be saved.
```bash
# Redirect the output to a file
extrack <path_to_input_file> > result.csv
# Use output argument
extrack <path_to_input_file> --output result.csv
```

For more information regarding available input arguments and flags please use `extrack --help`.

## License

Copyright (c) 2021-present [Viktor Franzén](https://github.com/vifraa)

Licensed under [GNU General Public License v3.0](./LICENSE)
