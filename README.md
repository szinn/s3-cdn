# s3-CDN - Serve CDN data out of an S3 bucket

## Environment Variables

- `S3CDN__HOST`: Host and port for the Minio server (e.g. <http://minio:9000>)
- `S3CDN__ACCESS_KEY_ID`: The access key id required to authenticate with the Minio server
- `S3CDN__SECRET_ACCESS_KEY`: The secret access key required to authenticate with the Minio server
- `S3CDN__BUCKET`: The bucket to serve data from
- `S3CDN__REGION`: The region for the Minio server.
- `S3CDN__PORT`: The port to listen to (defaults to 8080)

## Usage

```md
Usage: s3-cdn [OPTIONS] <COMMAND>

Commands:
server Start server
help Print this message or the help of the given subcommand(s)

Options:
-P, --port <PORT> Port S3-CDN should run on
-h, --help Print help
-V, --version Print version
```
