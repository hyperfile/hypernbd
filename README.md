# Hypernbd

A block device built on top of [nbdkit](https://gitlab.com/nbdkit/nbdkit) with Hyperfile.

## Overview

hypernbd project is a plugin for nbdkit, implement random read write block device over S3 directly.

## Prerequisites

### Install nbd user-space tools

On Amazon linux 2023, typically do following:

```
sudo dnf install nbd
```

### Install nbdkit

Please refer to [nbdkit](https://gitlab.com/nbdkit/nbdkit) repo for how to build nbdkit from source.

or, On Amazon linux 2023, typically do following:

```
sudo dnf install nbdkit
```

## How to build

```
cargo build --release
```

## Plugin for nbdkit

After successful build, you can find nbdkit plugin shared library `libhypernbd.so` in porject home drectory `target/release/`.

## How to run

### Create Hyperfile as NBD's backend storage with hypercli

```
# clone hypercli
git clone https://github.com/hyperfile/hypercli

# go to project directory
cd hypercli

# create nbd backend hyperfile

# create hyperfile with data block size in 512 KiB for example
cargo run --release -- file create-opt --data-block-size 524288 s3://mybucket/root/of/backend/

# set file length for hyperfile to ~9 TiB for example (data block storage space will be allocate when data block first write ONLY)
cargo run --release -- file set-len --size 10000000000000 s3://mybucket/root/of/backend/
```

### Run NBD server

| parameter | description |
| ---- | ---- |
| backend_url | S3 url of backend hyperfile root in format like `s3://mybucket/root/of/backend/` |
| backend_wal_url | S3 url of backend hyperfile root for WAL (Write Aread Log) in format like `s3://mybucket/root/of/wal/backend/`, set this parameter **ONLY** if you **REALLY** want to enable WAL |
| node_cache_config | in format "TYPE,PATH,SIZE,STRATEGY"<br> for example "LocalDisk,/tmp/hypernbd/,16777216,Recreate" |

```
# example of running in foreground and with rust log enabled at INFO level
RUST_LOG=info nbdkit -f --log=stderr --ipaddr=127.0.0.1 --port=7788 ./target/release/libhypernbd.so backend_url=<backend root S3 url> backend_wal_uri=<backend WAL root S3 url>
```

### Connect NBD device

```
# load NBD kernel module
sudo modprobe nbd

# connect to nbd server
sudo nbd-client 127.0.0.1 7788 /dev/nbd0
```

### Remove NBD device

```
# disconnect nbd0
sudo nbd-client -d /dev/nbd0
```

## Cleanup

### Remove WAL (optional)

```
# with aws-cli installed, do:
aws s3 rm --recursive s3://mybucket/root/of/wal/backend/
```

### Remove backend Hyperfile

```
# in hypercli directory, do:
cargo run --release -- file delete s3://mybucket/root/of/backend/
```

## License

This project is licensed under the Apache-2.0 License.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
