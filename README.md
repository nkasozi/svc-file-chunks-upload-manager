# File Upload Manager Service

## Table of Contents

- [About](#about)
- [Getting Started](#getting_started)
- [Usage](#usage)
- [Contributing](../CONTRIBUTING.md)

## About <a name = "about"></a>

A Dapr MicroService that splits uploads file chunks to different queues while they await reconcilaition.

## Getting Started <a name = "getting_started"></a>

Clone the repo

### Prerequisites

```
- Dapr
- Rust
```

### Installing

A step by step guide to get a development env running.

Run dapr

```
 daprd --app-id svc-file-chunks-upload-manager  --app-port 8084 --dapr-http-port 3600 --components-path "./dapr-components" --dapr-grpc-port 5006 --metrics-port 9091
```

Build the app

```
cargo build
```

Run the app

```
cargo run
```

### Running Tests

```
cargo test
```

Sample Read Comparison File Request

```
curl --location --request POST 'http://localhost:8082/read-file' \
--header 'Content-Type: application/json' \
--data-raw '{
  "upload_request_id": "RECON-TASK-1136275a-f81d-4843-91ea-8ed844e3fa35",
  "chunk_sequence_number": 1,
  "chunk_source": "PrimaryFileChunk",
  "chunk_rows": [
    {
      "raw_data": "0001, 20000, 10/02/2022",
      "row_number": 1
    },
    {
      "raw_data": "0002, 30000, 11/02/2022",
      "row_number": 2
    },
    {
      "raw_data": "0003, 40000, 12/02/2022",
      "row_number": 3
    }
  ],
  "is_last_chunk": true
}'
```
