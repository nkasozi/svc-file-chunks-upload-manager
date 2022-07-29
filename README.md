# File Upload Manager Service

## Table of Contents

- [About](#about)
- [Getting Started](#getting_started)
- [Usage](#usage)
- [Contributing](../CONTRIBUTING.md)

## About <a name = "about"></a>

A Dapr MicroService that provides CRUD functionality for ReconTasks.

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
daprd --app-id svc-task-details-repository-manager  --app-port 8080 --dapr-http-port 3500 --components-path "./dapr-components" --dapr-grpc-port 5005
```

Build the app

```
cargo build
```

Run Tests

```
cargo test
```

Run the app

```
cargo run
```

Sample Create ReconTasks Request

```
curl --location --request POST 'http://127.0.0.1:8080/task-details' \
--header 'Content-Type: application/json' \
--data-raw '{
    "user_id": "test-user-id",
    "source_file_name": "test-src-file",
    "source_file_hash": "test-src-file-hash",
    "source_file_row_count": 25,
    "source_file_column_count": 10,
    "comparison_file_name": "test-cmp-file",
    "comparison_file_hash": "test-cmp-file-hash",
    "comparison_file_column_count": 30,
    "comparison_file_row_count": 10,
    "recon_configurations": {
        "should_check_for_duplicate_records_in_comparison_file": false,
        "should_reconciliation_be_case_sensitive": false,
        "should_ignore_white_space": true,
        "should_do_reverse_reconciliation": false
    },
    "comparison_pairs": []
}'
```

GET previously created task 
```
curl --location --request GET 'http://127.0.0.1:8080/task-details/RECON-TASK-05aecf16-cf0a-40f6-8af7-fd6ef7e89d70'
```

## Usage <a name = "usage"></a>

Add notes about how to use the system.
