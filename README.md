### Setup

- run `docker compose up`
- run `cp .env.example .env`
- run `diesel setup`

### Run app

```sh
cargo run
```

### Migrations

Create migration
```sh
diesel migration generate <name>
```

Run migrations
```sh
diesel migration run
```