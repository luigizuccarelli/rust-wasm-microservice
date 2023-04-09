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
