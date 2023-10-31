# sfproc
[![license](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](https://github.com/r1cm3d/gk-yaml/blob/master/LICENSE)

## Prerequisites
[![rust](https://img.shields.io/badge/rust-2021-orange?style=flat-square)](https://github.com/rust-lang/rust)
``` console
brew install rustup-init && rustup-init
```

## Table of Contents
* [Prerequisites](#prerequisites)
* [About](#about-the-project)
* [Building](#building)
* [Installing](#installing)
* [Usage](#usage)
* [Testing](#testing)
* [Getting Help](#getting-help)

## About
This CLI application is responsible to process settlement files using AWS Simple Storage Service (S3) as repository.

## Building
```
make build
```
It will call `cargo build` in order to download and build all dependencies.

## Installing
```
make install
```
It requires `SFPROC_BIN` environment variable that must be on `PATH` environment variable. The user must have permissions to
write in this directory.

## Usage
This application basically copies files from S3 bucket to another attaching some metadata and encrypting them with a Key Management Service (KMS) key if is needed.

### To copy all files from bucket MY_BUCKET to the same bucket:
``` console
./sfproc \
    --bucket MY_BUCKET \
    --endpoint EDID \
    --prefix S3_OBJECT_KEY \
    --regex REGULAR_EXPRESSION_TO_FILTER_FILES \
    --kms-key KMS_KEY_ARN_TO_ENCRYPTED_FILES \
    --verbose
```

The arguments `--regex`, `--verbose` and `--kms-key` are not mandatory.

## Testing
```
make test
```
It will call `cargo test` aiming to run the basic unit tests.

## Getting Help

```console
./sfproc --help
```

Help information will be displayed:

```console
sfproc - settlement files processor 0.2.3
A CLI application that is responsible to process settlement files.

USAGE:
    sfproc [OPTIONS] --endpoint <ENDPOINT> --bucket <BUCKET> --prefix <PREFIX>

OPTIONS:
    -b, --bucket <BUCKET>        S3 bucket to look up
    -e, --endpoint <ENDPOINT>    Endpoint/CIB related to the file
    -h, --help                   Print help information
    -k, --kms-key <KMS_KEY>      The ARN of the KMS key that must be used to encrypt the sensible
                                 files
    -p, --prefix <PREFIX>        The prefix to be applied in the look up in order to avoid
                                 unnecessary requests
        --pretend                Enable pretend mode. In the pretend mode, the files will not be
                                 copied. This option is useful to validate the --regex option
    -r, --regex <REGEX>          The base regex pattern to look up into storage repository
    -s, --suffix <SUFFIX>        The suffix to be applied in the end of the file name. This option
                                 is DANGEROUS and might skip the integrity validation
    -v, --verbose                Enable DEBUG log mode
    -V, --version                Print version information
