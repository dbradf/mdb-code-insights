# Code Insights in MongoDB

Load your git commit data into a MongoDB instance to generate reports on your data.

## Usage

First, create a config file to store details of how to store the data. This file should be YAML
and should contain the following keys:

```yaml
mongo_uri: mongodb://localhost:27017
database: code_insights
collection: mongodb
```

- **mongo_uri**: URI to connect to the MongoDB instance to store the data.
- **database**: Name of database to store the data in.
- **collection**: Name of collection to store the data in.

Once you have a configuration file, you can use the `load` subcommand to load git data into the
specified MongoDB instance:

```bash
mdb-code-insights --config-file config.yml my-repo load --after-date 2018-01-01 --repo-dir path/to/repo
```

- **--after-date**: Load commits created after the given date.
- **--repo-dir**: Directory containing the git repository to analyze.

Once the data has been loaded, you can use things like aggregations and charts to analyze it. There
are a few aggregations built into the tool.

### File activity

The file activity aggregation will show which files have been the most actively since a given
date. You can use this to find hotspots in the code that may benefit from refactoring.

The following aggregation will provide that data:

```
[
    {
        $match: {
            date: {$gt: ISODate("2020-01-01")},
        },
    },
    {
        $unwind: {
            path: "$files",
        },
    },
    {
        $sortByCount: "$files.filename",
    },
]
```

You can use the `mdb-code-insights` tool to run this aggregation for you and display the results:

```bash
$ mdb-code-insights --config-file config.yml file-activity --since 2020-01-01
src/third_party/wiredtiger/import.data: 1335
etc/evergreen.yml: 892
src/mongo/db/SConscript: 354
src/mongo/db/repl/replication_coordinator_impl.cpp: 309
src/mongo/db/s/SConscript: 299
src/mongo/db/repl/SConscript: 245
SConstruct: 226
src/third_party/wiredtiger/test/evergreen.yml: 223
src/third_party/wiredtiger/src/include/wiredtiger.in: 220
...
```

For large repositories, you may wish to only look at part of the repository, we can add a
filter in our aggregation to do just that (you can use the `--prefix` option from the command line
to include this).

```
[
    {
        $match: {
            date: {$gt: ISODate("2020-01-01")},
        },
    },
    {
        $unwind: {
            path: "$files",
        },
    },
    {
        $match: {
            "files.filename": {
                $regex: "^src/mongo/db",
            }
        }
    },
    {
        $sortByCount: "$files.filename",
    },
]
```

### File coupling

Once we are aware of an active file, we might want to see how other files are coupled to it. We can
use the following aggregation to gather that data:

```
[
    { 
        "$match": { 
            "date": {"$gt": ISODate("2020-01-01")}, 
            "files.filename": "src/mongo/db/repl/replication_coordinator_impl.cpp" 
        } 
    },
    { 
        "$facet": { 
            "total_commits": [{ "$count": "commit" }], 
            "seen_with": [
                { 
                    "$unwind": { "path": "$files" } 
                }, 
                { 
                    "$match": { 
                        "files.filename": { "$ne": "src/mongo/db/repl/replication_coordinator_impl.cpp" } 
                    } 
                }, 
                { 
                    "$group": {
                        "_id": "$files.filename", 
                        "count": { "$sum": 1 } 
                    } 
                }, 
                { 
                    "$sort": { "count": -1 } 
                }
            ] 
        } 
    },
]
```

We can use the command line `file-coupling` subcommand to perform this aggregation:

```bash
$ mdb-code-insights --config-file config.yml file-coupling --since "2020-01-01" --filename src/mongo/db/repl/replication_coordinator_impl.cpp
src/mongo/db/repl/replication_coordinator_impl.cpp: 309 instances

 - src/mongo/db/repl/replication_coordinator_impl.h: 91: 29.45%
 - src/mongo/db/repl/replication_coordinator_mock.cpp: 63: 20.39%
 - src/mongo/embedded/replication_coordinator_embedded.cpp: 62: 20.06%
 - src/mongo/db/repl/replication_coordinator_mock.h: 61: 19.74%
 - src/mongo/embedded/replication_coordinator_embedded.h: 60: 19.42%
 - src/mongo/db/repl/replication_coordinator_noop.h: 60: 19.42%
 - src/mongo/db/repl/replication_coordinator_noop.cpp: 60: 19.42%
 - src/mongo/db/repl/replication_coordinator.h: 59: 19.09%
 - src/mongo/db/repl/topology_coordinator.cpp: 57: 18.45%
 - src/mongo/db/repl/replication_coordinator_impl_test.cpp: 51: 16.50%
 - src/mongo/db/repl/replication_coordinator_impl_heartbeat.cpp: 47: 15.21%
 ...
```

### File ownership

It can also be useful to check if a file has a clear owner. We can use the following aggregation
to check who has been changing a file the most:

```
[
    { 
        $match: { date: { $gt: ISODate("2020-01-01") } } 
    },
    { 
        $unwind: { path: "$files" } 
    },
    { 
        $match: { "files.filename": "src/mongo/db/repl/replication_coordinator_impl.cpp" } 
    },
    {
        $sortByCount: "$author" 
    },
]
```

Again we can use the command line tool to perform this aggregation with the `file-ownership`
subcommand:

```bash
$ mdb-code-insights --config-file config.yml file-ownership --since "2020-01-01" --filename src/mongo/db/repl/replication_coordinator_impl.cpp 
Owners of src/mongo/db/repl/replication_coordinator_impl.cpp: 309 total changes
William Schultz: 23 (7.44%)
Lingzhi Deng: 17 (5.50%)
Pavi Vetriselvan: 17 (5.50%)
A. Jesse Jiryu Davis: 16 (5.18%)
Jason Chan: 15 (4.85%)
Matthew Russotto: 15 (4.85%)
Vesselina Ratcheva: 14 (4.53%)
...
```

## Inspiration

* [Code Maat](https://github.com/adamtornhill/code-maat)
