# Overview

A simple json http wasm (webAssembly) microservice used as POC for wasmedge/wasmruntime backend profiling

## Usage

Clone the repo 

```bash

git clone git@github.com:luigizuccarelli/rust-wasm-microservice.git

cd rust-wasm-microservice

cargo build --target=wasm32-wasi --release

# you can also build for x86
# comment out these lines in the Cargo.toml file

#hyper_wasi = { version = "0.15", features = ["full"]}
#tokio_wasi = { version = "1.21", features = ["rt", "macros", "net", "time", "io-util"]}

cargo build --target=x86_64-unknown-linux-gnu --release

```

Start the service

```bash

target/x86_64-unknown-linux-gnu/release/rust-wasm-microservice

```

Load test the service

Clone the load testing repo 

```bash

git clone git@github.com/fcsonline/drill

cd drill

cargo build --target=x86_64-unknown-linux-gnu --release

```

Create a benchmark yaml file

```bash
cat <<EOF >>benchmark.yaml
base: 'http://<ip-of-rust-wasm-microservice>:8080'
iterations: 2100
concurrency: 100
rampup: 2

plan:
  - name: POSTJSON
    request:
      url: /streamdata
      method: POST
      body: '{
        "name":"iot-paas",
        "deviceId":"id123444",
        "patientId":"pid3423423",
        "data":[
          {
            "hr":80,
            "bps":120,
            "bpd":80,
            "spo2":96,
            "custom":{
              "tp":34.7,
              "rr":20,
              "etc":"123"
            },
            "date":"17-03-2023"
          }
        ]
      }'
      headers:
        Content-Type: 'application/json'
EOF
```

Execute the load test

```bash

target/x86_64-unknown-linux-gnu/release/drill  --benchmark benchmark.yaml --stats --quiet

```

Update the benchmark file to create more load (change concurrency & iterations)

- be amazed

## Results (without wasm - native on host)

Using 100 concurrent connections and 4200 iterations

```bash

$ target/release/drill  --benchmark config.yaml --stats --quiet

Concurrency 100
Iterations 4200
Rampup 2
Base URL http://192.168.0.29:8080


POSTJSON                  Total requests            4200
POSTJSON                  Successful requests       4200
POSTJSON                  Failed requests           0
POSTJSON                  Median time per request   1ms
POSTJSON                  Average time per request  1ms
POSTJSON                  Sample standard deviation 1ms
POSTJSON                  99.0'th percentile        5ms
POSTJSON                  99.5'th percentile        5ms
POSTJSON                  99.9'th percentile        6ms

Time taken for tests      0.1 seconds
Total requests            4200
Successful requests       4200
Failed requests           0
Requests per second       28674.44 [#/sec]
Median time per request   1ms
Average time per request  1ms
Sample standard deviation 1ms
99.0'th percentile        5ms
99.5'th percentile        5ms
99.9'th percentile        6ms


```

Without fine tuning (use string buffers instead of deserializing & serializing json paylods)

Tested on amd ryzen9 with 32G RAM - 28674 requests/sec

This is extremely performant

## Docker Build

There are 2 dockerfiles in the repo one to build a minimal wasm container
and another to build a minimal container with glibc dependencies for the rust binary

The later has a size of around 32MB which is extremely light. I also did some profile testing on it
with results very similar to the raw native verison but with a performance hit of around 20% (using podman run)
