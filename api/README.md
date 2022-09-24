# Organization API

This project aims to deploy an API software based on REST to manage user on SaaS.


## Run

```
docker run --name organization -p 5432:5432 -e POSTGRES_PASSWORD=postgres -e POSTGRES_DB=organization -d postgres
cargo build
./target/debug/organization --database-url=postgres://postgres:postgres@localhost:5432/organization
```

## CLI

```
./target/debug/organization-cli -s http://localhost:8080 organization list
```


# Note Dockerfile is built for aarch64
To change for x86_64 juste replace it in Dockerfile
