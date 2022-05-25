---
weight: 3
---
{{< hint info >}}
Before performing this analysis, you will want to have a [active file to target]({{< relref "file_activity.md" >}})
{{< /hint >}}

From the previous step, we decided to target the file `src/mongo/db/repl/replication_coordinator_impl.cpp`
and see how it is coupled to other files in the repository.

## Using an aggregation

We can analyze the file coupling with the following aggregation:

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
                    "$sortByCount": "$files.filename"
                },
            ] 
        } 
    },
]
```

## Using the command line

We can also use the `file-coupling` subcommand to perform this aggregation:

```bash
mdb-code-insights --config-file config.yml file-coupling --since "2020-01-01" --filename src/mongo/db/repl/replication_coordinator_impl.cpp
```

```bash
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

From the output, a lot of the files look to be related to the file we are analyzing. Most of them
look like tests or header files that we would expect to be closely related. However, the
`src/mongo/db/repl/topology_coordinator.cpp` file seems to stand out. This file seems to change
along with the given file more frequently than the unit tests for that file. That might be something
we would want to look into.

## Next steps

Now that we have analyzed file coupling, let's try another analysis to see
[file ownership]({{< relref "file_ownership.md" >}})
