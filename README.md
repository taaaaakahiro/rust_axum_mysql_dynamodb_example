# stock-metrics

Stock price and stats viewer.

## Getting Started

### Middleware

Launch the middleware by executing docker compose:

```
cd local-middleware
docker compose up -d
```

This app now takes advantage of the following tools:

- MySQL
- DynamoDB (future)

### Setting up database tables

Please run SQLs in `migrations` directory. `up.sql` can be up tables, `down.sql` removes them.

### Run the app

```
cargo run
```

After running the command, you can see tracing logs.

```
    Finished dev [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/bootstrap`
2021-12-21T13:49:54.407374Z  INFO stock_metrics_driver::startup: Server listening on 127.0.0.1:8080
```

## Architecture

This example has 4 workspaces as following:

- stock-metrics-driver (driver or controller)
- stock-metrics-app (app or usecase)
- stock-metrics-kernel (kernel or domain)
- stock-metrics-adapter (adapter or infrastructure)
