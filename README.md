# lbasedb-api

REST API interface for [Lbasedb](http://crates.io/crates/lbasedb) DBMS.

Source code: https://github.com/fomalhaut88/lbasedb-api

## About Lbasedb

`lbasedb` is a powerful low level DBMS that is focused on dataset structure.
The algorithms are optimized for the compact data storage and for high
performance on get and append operations. Particularly, due to this, 
deleting or indexing are not supported. The allowed data types are also
limited (integers, floats and bytes) for making easy integration with
C-like or similar common interfaces (like Python, CUDA, JSON and so on).
The database has asynchronous access to the entities powered by `tokio`.
It is supposed to be used for the data that have billions and more records
and thousands columns of simple data types that must be appended without
extra overhead.

Reference: http://crates.io/crates/lbasedb

Source code: https://github.com/fomalhaut88/lbasedb

## Run in Docker

1. Pull the docker image:

```
docker pull fomalhaut88/lbasedb
```

2. Run the container:

```
docker run \
    -p 8080:8080 \
    --restart=always \
    --ulimit nofile=1024:1024 \
    --volume /tmp/lbasedb:/app/db \
    --name lbasedb-app \
    --env WORKERS=16 \
    -d fomalhaut88/lbasedb
```

Notice, since Lbasedb keeps an open file for each column, you may need to  
increase the maximum number of open files allowed (see `ulimit` for details), 
that is usually 1024 by default. In the example above, the limit is set to 
1024 as default but it can be greater.

## API description

| Path | Method | Description | Params | Input body example | Output body example |
|---|---|---|---|---|---|
| `/version` | `GET` | Version of API. | | | `0.1.2` |
| `/data` | `GET` | Get JSON dataset. | `feed: str` - name of the feed <br> `ix: int` - start index <br> `size: int` - number of rows <br> `col: [str]` - names of columns | | `{"x": [2, 3]}` |
| `/data` | `POST` | Push JSON dataset into the end. | `feed: str` - name of the feed | `{"x": [2, 3]}` | |
| `/data` | `PUT` | Update JSON dataset from the index. Columns not listed will be zeroed. | `feed: str` - name of the feed <br> `ix: int` - start index | `{"x": [2, 3]}` | |
| `/data` | `PATCH` | Update JSON dataset from the index. Columns not listed will stay the same. | `feed: str` - name of the feed <br> `ix: int` - start index| `{"x": [2, 3]}` | |
| `/size` | `GET` | Get size of the feed. | `feed: str` - name of the feed | | `{"size": 2}` |
| `/size` | `PUT` | Resize the feed with all its columns. Last records will be removed or zero records appeared in the end. | `feed: str` - name of the feed | `{"size": 2}` | |
| `/raw` | `GET` | Get raw bytes in the column. | `feed: str` - name of the feed <br> `ix: int` - start index <br> `size: int` - number of units <br> `col: str` - name of columns | | binary |
| `/raw` | `POST` | Insert raw bytes into the column. | `feed: str` - name of the feed <br> `ix: int` - start index <br> `col: str` - name of columns | binary | |
| `/feed` | `GET` | List available feeds. | | | `[{"name": "xyz"}]` |
| `/feed` | `POST` | Add a new feed. | | `{"name": "xyz"}` | |
| `/feed` | `PATCH` | Rename the feed. | `name: str` - name of the feed | `{"name": "xyz2"}` | |
| `/feed` | `DELETE` | Delete the feed. | `name: str` - name of the feed | | |
| `/col` | `GET` | List available columns in the feed. | `feed: str` - name of the feed | | `[{"name": "y", "datatype": "Int64"}]` |
| `/col` | `POST` | Add a new column. | `feed: str` - name of the feed | `{"name": "x", "datatype": "Int64"}` | |
| `/col` | `PATCH` | Rename the column. | `feed: str` - name of the feed <br> `name: str` - name of the column | `{"name": "y"}` | |
| `/col` | `DELETE` | Delete the column. | `feed: str` - name of the feed <br> `name: str` - name of the column | | |

## Environment variables

| Variable | Description | Default |
|---|---|---|
| `DATA_PATH` | Path to the directory for the data. | `./tmp/db` |
| `WORKERS` | Number of workers to process. | `1` |
| `HOST` | Host to deploy. | `localhost` |
| `PORT` | Port to deploy. | `8080` |
