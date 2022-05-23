---
weight: 2
---
{{< hint info >}}
Before performing this analysis, before you have some data [loaded from a git repository]({{< relref "loading_data.md" >}})
{{< /hint >}}

Now that we have some data in our mongo instance, lets start analyzing it. A useful place to start
it to see which files have been changed the most. Files that change frequently are more likely to
contain bugs or be bottlenecks in the development process.

## Using an aggregation

We can use the following aggregation to find frequently changed files:

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

## Using the command line

We can also use the `file-activity` subcommand to run this aggregation:

```bash
mdb-code-insights --config-file config.yml file-activity --since 2020-01-01
```

```bash
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

Looking at the output, the first source code file that show up is `src/mongo/db/repl/replication_coordinator_impl.cpp`
with 309 changes.  We will use this file for future exploration.

## What's next

Now that have an active file, we can analyze what other files are [tightly coupled]({{< relref "file_coupling.md" >}}) to it.
