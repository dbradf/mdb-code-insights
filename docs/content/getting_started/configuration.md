---
weight: 2
---
There are a few pieces of information needed to run the tool. These can be provided via
command line arguments or with a configuration file.

The required details are:

* **mongo_uri**: The URI to use to connect to mongodb.
* **database**: The database in the mongo instance to store/load data.
* **collection**: The collection in the database to store/load data.

## Command line configuration

All three options mentioned can be provided as command line options (see `mdb-code-insights --help`
for details). They should be specified before the subcommand is given.

For example,

```bash
mdb-code-insights --mongo-uri "mongodb://localhost:27017" --database "code_insights" --collection "my_repo" load ...
```

## Configuration file configuration

If you prefer, you can store the configuration options in a YAML file and provide that to the tool.

The YAML file should look like the following:

```yaml
mongo_uri: mongodb://localhost:27017
database: code_insights
collection: mongo
```

The `--config-file` option can be used to specify the file to use:

```bash
mdb-code-insights --config-file config.yml load ...
```
