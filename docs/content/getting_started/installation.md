---
weight: 1
---
## Prerequisites

In order to use this tool, you will need the following:

* [git](https://git-scm.com) command line: Version 2.17 or higher
* [MongoDB](https://www.mongodb.com): Version 5.0 or higher. Most features of the tool will work
  with a [local instance](https://www.mongodb.com/try/download/community) of MongoDB. In order
  to work with Chart's visualizations, you will need to use an
  [Atlas cluster](https://www.mongodb.com/try).

## Install pre-built binaries

Pre-built binaries for Linux, Mac, and Windows are available [here](https://github.com/dbradf/mdb-code-insights/releases/latest).

To install, download the appropriate binary for your system to somewhere that is in your PATH. You
should then be able to execute it from the command line.

## Install from source

If you would prefer to build from source, be sure you have a [Rust](https://www.rust-lang.org/learn/get-started)
development environment installed. You can then use `cargo build --release` to build the tool.

## Testing your install

You can test your install with the following command, `mdb-code-insights --help`. You should see
a message like the following:

```bash
$ mdb-code-insights --help
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
    file-activity       
    file-coupling       
    file-ownership      
    files-per-commit    
    help                Print this message or the help of the given subcommand(s)
```

## What's next

Once you have the tool installed, you will want to [configure it]({{< relref "configuration.md" >}}).
