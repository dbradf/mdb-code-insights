---
weight: 0
---
This demo will walk through using the `mdb-code-insights` tool to analyze a git repository. If you
want to follow along, be sure to have the tool [installed]({{< relref "../getting_started/installation.md" >}})
and [configured]({{< relref "../getting_started/configuration.md" >}}).

The demo will walk through an analysis of the [mongodb/mongo](https://github.com/mongodb/mongo)
repository, but the steps should work with any git repository.

For this demo, we will store our configuration in a file called "config.yml". It should look similar
to this (feel free to customize it to your liking):

```yaml
mongo_uri: mongodb://localhost:27017
database: code_insights
collection: mongo
```

## Aggregation Framework

Throughout this demo, we will use MongoDB's [Aggregation Framework](https://www.mongodb.com/basics/aggregation)
to explore the data. Most of the aggregation can be run via the `mongodb-code-insights` tool, but
if you would like to run them manually or tweak them, you may want to have a mongo client available.

[MongoDB Compass](https://www.mongodb.com/products/compass), [mongosh](https://www.mongodb.com/docs/mongodb-shell/)
or any other mongo client should work for the examples in this demo.

## Getting Started

We will start by [loading our git data into mongo]({{< relref "loading_data.md" >}}).
