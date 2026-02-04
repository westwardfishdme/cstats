# cstats
Calculates statistics data in Rust. Accepts csv and plaintext files as stdin or files.

## Usage
- `cstats stats -i=csv {subcommand} {file}` parse the file as a csv-- currently only supports single column reading, (multi-column soon to come)
- `cstats stats all` shows all information
- `cstats stats sum` shows the sum

Todo:
- fix `STDIN` parsing, doesn't work at the moment...
- `cstats stats max {file}` show the max
- `cstats stats min {file}` show the min
