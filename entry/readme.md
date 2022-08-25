# The Entry Project

by: ExaltedToast

The Entry Project is designed to be a simple API that covers the major facets of designing a web API in as little complexity as possible.
The project is to make a simple JSON api in a language/framework of choice and covers topics like:

- Primitive Types
- Datetimes
- UUIDs
- Routing endpoints
- Simple database access
- Error handling

The goal of this project is to enable a large amount of education on basic API topics with very little complexity.
A future version of this document may have more requirements if more critical API operations are recognized.

## The Entry API

The Entry API must have three endpoints:

- Healthcheck
- GET an entry
- POST an entry

All endpoints except optionally `Healthcheck` should be based on the JSON format.
Entries match the following format:

```json
{
	"id": A UUID,
	"timestamp": A timestamp in UTC format,
	"content": An arbitrary string,
	"priority": An optional nonnegative integer
}
```

For example:

```json
{
	"id": "661520c3-3801-4963-b3c5-d67aa6b4c5ab",
	"timestamp": "2022-08-20T05:29:26Z",
	"content": "This is an entry for The Entry Project",
	"priority": 2
}
```

### Healthcheck

The API must have a healthcheck endpoint to determine the status of the application.

- The healthcheck should be available at `/api/healthcheck`.
- The response should be any string containing `HEALTHY`.

### GET an Entry

The API must have a mechanism for retrieving an Entry.

- The GET endpoint should be `/api/<id>` where `id` is a UUID.
- If `id` is not a correctly formatted UUID, return a `400`.
- If `id` is not present in the database, return a `404`.

### POST an entry

The API must have a mechanism for creating an Entry.

- The POST endpoint should be at `/api/`.
- The POST body must be an Entry containing `content`.
- An `id` should be randomly generated (UUIDv4).
- A `timestamp` should also be generated (current time).
- These exception cases must be handled:
	- If `id` or `timestamp` are included in the request, they should be ignored and overwritten.
	- If `content` is missing, return a `400`.
	- If `priority` is missing, it should be initialized to `0`.
	- If `priority` is negative, return a `400`.
- These exception cases can be optionally handled:
	- If `priority` is too large to be stored in your language's integer type, return a `400`.
	- If any other fields are present, return a `400`.
	- If a duplicate UUID is generated (just in case...).

### Database

The API must have a mechanism for persisting entries.

- The entries should be persisted in a relational database.
	- That is, some flavor of SQL, typically Sqlite.
- The storage must be preseved between program runs.
- The primary key for the `entries` table must be `UUID`.
- The `timestamp` must be stored in a database-native datetime format.