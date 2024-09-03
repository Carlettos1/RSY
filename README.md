# How to run

## Installs

To install SurrealDB, use:

```sh
curl -sSf https://install.surrealdb.com | sh
```

and then move surreal to `/usr/local/bin` or add `~/.surrealdb/surreal` to `$PATH`

```sh
sudo mv ~/.surrealdb/surreal /usr/local/bin
```

To install Trunk, use:

```sh
cargo install --locked trunk
```

Also install `wasm-bindgen-cli` and `wasm32-unknown-unknown`.

```sh
cargo install --locked wasm-bindgen-cli
rustup target add wasm32-unknown-unknown
```

## Run database, api, and web app

On db/ run:

```sh
surreal start file:storage/main.db -A --auth -u root -p root
```

On carlettos_api/ run:

```sh
cargo watch -c -q -x "run -r"
```

On carlettos_web/ run:

```sh
trunk serve --open --port 8081 --release
```

## To Update / Create the surrealdb
run, on test, on `db/`:
```sh
surreal import --namespace root --database database -p root -u root --endpoint http://localhost:8000 init.surreal
```
on production, use the production endpoint.