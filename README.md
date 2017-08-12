### Simple redis test

To build, use cargo to build the target:

```bash
cargo build --target=x86_64-unknown-linux-musl
```

This will create a new debug executable in the target directory.

Then build the docker:

```bash
docker-compose build
```

This will create the image and copy the build targets into it. You can then run the executable like so:

```bash
$ docker-compose run rust target/x86_64-unknown-linux-musl/debug/redis-test
Starting redistest_redis_1 ... done
creating client
grabbing connection
setting a value
getting the value
retrieved value: value
```