# Code Insights in MongoDB

Load your git commit data into a MongoDB instance to generate reports on your data.

[![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/dbradf/mdb-code-insights)](https://github.com/dbradf/mdb-code-insights/releases/latest)
[![Documentation](https://img.shields.io/badge/Docs-Published-green)](https://dbradf.github.io/mdb-code-insights/)

## Documentation

Documentation can be found [here](https://dbradf.github.io/mdb-code-insights/).

## Usage

```
mdb-code-insights 

USAGE:
    mdb-code-insights [OPTIONS] <SUBCOMMAND>

OPTIONS:
        --collection <COLLECTION>      Collection to use
        --config-file <CONFIG_FILE>    Path to config file to use
        --database <DATABASE>          Database to use
    -h, --help                         Print help information
        --mongo-uri <MONGO_URI>        URI to mongodb instance

SUBCOMMANDS:
    file-activity       Generate a report on the most active files
    file-coupling       Generate a report on file-coupling for the given file
    file-ownership      Generate a report on the author ownership of the given file
    files-per-commit    Generate a report on the average number of files per commit by author
    help                Print this message or the help of the given subcommand(s)
    load                Load data from git into a mongo instance
```

## Inspiration

* [Code Maat](https://github.com/adamtornhill/code-maat)
