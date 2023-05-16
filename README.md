# tabi_plan
たびぷらん

## Requirements
- [Okteto CLI](https://www.okteto.com/docs/getting-started/#installing-okteto-cli)
- Docker
- Rust 1.69

## Preparation
- Prepare API Key that ChatGPT
- Prepare API Key that Google Maps

## Usage

Run API Srever
```sh
$ make run
```

Build docker image and push Okteto.dev's registry
```sh
$ make image-build
```
