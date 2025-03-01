# rdb/relational

RDB has to main endpoints: server and client.

The server services the client by dealing with requests in SQL.
It implements:

- Query parsing, planning & execution
- Storage

The client queries the server and presents the information to the user. It's responsible for:

- Fetching
- Formatting

## Server

The server is implemented as a gRPC service. See [`service.proto`](../schema/service.proto) for the exact schema.
It has only one RPC, `Run`, which accepts an SQL query along with parameters, and returns rows as `google.protobuf.Any`.

Rows are dynamically created protobuf types that match the return schema from the query.

### Query execution

Queries have the following shapes of today:

- `INSERT` queries: only `INSERT INTO <table> VALUES (<row1-comma-separated-constants>)[,...,(<rowN-comma-separated-constants>)];`.
- `UPDATE` queries: only `UPDATE <table> SET <field1>=<constant1>[,...,<fieldN>=<constantN>] WHERE <conditions>`.
- `DELETE` querise: only `DELETE FROM <table> WHERE <conditions>`;
- `SELECT` queries: only `SELECT <fields> FROM <table> WHERE <conditions>;`
- Values must be known at query time; using other fields, or functions are not supported at the moment.

1. Queries are parsed using the [`sqlparser`](https://docs.rs/sqlparser/latest/sqlparser/) crate into an AST.
1. A `Planner` formulates the execution plan for the query.
1. The plan is executed against a `Storage`.
1. The response is streamed back to the client.

### Query planning

Query planning is done using a `Planner`. The `Planner` converts the SQL AST into an `Operation` tree.
There are two kinds of `Operation`s:

- `Pull` and `Transform`. `Pull` operations are leafs of the `Operation` tree, and they pull data from `Storage`.
- `Transform` operations do some kind of transformation on the input rows and produce output rows.

### Storage

Each table creates three binary files for the storage:

1. Table metadata (see [`table/metadata.proto`](../schema/table/metadata.proto)), contains information about the table schema, name, etc...
1. Rows file, contains the table rows as concatenated serialized protobuf messages according to the message type of the table.

## Client

The client is a CLI that submits queries and formats the result as CSVs.
