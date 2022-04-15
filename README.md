# Code Insights in MongoDB

Load your git commit data into a MongoDB instance to generate reports on your data.

## Usage

First, use the `load` subcommand to load git data into a MongoDB instance:

```bash
mdb-code-insights --mongo-uri mongodb://localhost:27017 --database codeinsights --collection my-repo load --after-date 2018-01-01 --repo-dir path/to/repo
```

Once the load has been loaded, you can use things like aggregations and charts to analyze it. There
are a few aggregations built into the tool.

You can use `files-per-commit` to see how many files on average an author touches per commit:

```bash
mdb-code-insights --mongo-uri mongodb://localhost:27017 --database codeinsights --collection my-repo files-per-commit
user-1(4): 35.25
user-2(3): 9
user-3(1): 9
user-4(156): 8.679487179487179
user-5(423): 8.678486997635934
user-6(55): 8.10909090909091
...
```

You can use `file-coupling` to see how tightly coupled a file is with other files:

```bash
mdb-code-insights --mongo-uri mongodb://localhost:27017 --database codeinsights --collection my-repo file-coupling --filename src/project/file_0.rs
src/project/file_0.rs: 94 instances

 - tests/project/test_file_0.rs: 44: 46.81%
 - tests/project/test_file_1.rs: 26: 27.66%
 - src/project/file_1.rs: 24: 25.53%
...
```

## Inspiration

* [Code Maat](https://github.com/adamtornhill/code-maat)
