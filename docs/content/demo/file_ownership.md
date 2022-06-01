---
weight: 4
---
{{< hint info >}}
Before performing this analysis, you will want to have a [active file to target]({{< relref "file_activity.md" >}})
{{< /hint >}}

One interesting view of a file is who has been making changes to it. This can be particularly
useful when there are questions about the file or changes to it that need to be reviewed. We can
analyze the file ownership to determine who has made the most changes to a file.

## Using an aggregation

We can analyze the file ownership with the following aggregation:

```
[
    { 
        $match: { 
            date: { $gt: ISODate("2020-01-01") },
            "files.filename": "src/mongo/db/repl/replication_coordinator_impl.cpp", 
        } 
    },
    {
        $sortByCount: "$author" 
    },
]
```

## Using the command line

We can also use the `file-ownership` subcommand to perform this aggregation:

```bash
mdb-code-insights --config-file config.yml file-ownership --since "2020-01-01" --filename src/mongo/db/repl/replication_coordinator_impl.cpp 
```

```
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

It looks like this file has had a good number of people make changes to it over the given time
frame. It is good that there is some distributed knowledge about the file, but it could also be
a sign that this file has too broad of responsibilities and might be a good target to refactor and
split up.

## What's next

Now that you've looked at file ownership, try [visualizing some data with MongoDB Charts]({{< relref "using_charts.md" >}}).
