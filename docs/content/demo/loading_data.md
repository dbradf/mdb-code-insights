---
weight: 1
---
{{< hint info >}}
Before starting this demo, before to have the prerequisites installed as described in the
[intro]({{< relref "intro.md" >}}).
{{< /hint >}}

The first thing we need to do is load data from our git repository into our MongoDB instance. This
is done with the `load` subcommand. You will need a local copy of the git repository you wish to
analyze before starting. For the demo, we will assume that the repo has been cloned to `$HOME/repos/mongo`.

## Using the load command

The load subcommand will load data from the specified git repository into our mongo instance. It
takes 2 arguments:

- **--repo-dir**: The path to the git repository to analyze.
- **--after-date**: How far back in the repository to collect information for. For repositories with
  a lot of history, this can speed up the import time by restricting the amount of data that will be
  gathered. Additionally, if you have a job running the tool on a regular basis to keep the mongo data
  up to date, this option can be used to only look at new git data.

We can load the last few years of git data from the mongo repository with:

```bash
mdb-code-insights --config-file config.yml load --after-date 2018-01-01 --repo-dir $HOME/repos/mongo
```

Depending on how much git data is being collected, the command could take a few minutes to run. Once
it has completed, the output should look something like this:

```bash
$ target/release/mdb-code-insights --config-file config.yml load --after-date 2018-01-01 --repo-dir $HOME/repos/mongo
Create data in: 38156ms
Loaded 24994 commits!
Sent data to mongo in: 2170ms
```

If you now look at your mongo instance, the git data should be available to explore.

## Next Steps

Once the data has been loaded, we can start to analyze it. We will start by finding the
[most heavily edited files]({{< relref "file_activity.md" >}}).
