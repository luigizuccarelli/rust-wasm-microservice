use hyper::body;
use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{Body, Error, Method, Request, Response};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub name: String,
    pub device_id: String,
    pub patient_id: String,
    pub data: Vec<Daum>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Daum {
    pub hr: i64,
    pub bps: i64,
    pub bpd: i64,
    pub spo2: i64,
    pub custom: Custom,
    pub date: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Custom {
    pub tp: f64,
    pub rr: i64,
    pub etc: String,
}

/// handler - reads json
async fn process_payload(req: Request<Body>) -> Result<Response<Body>, Error> {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/streamdata") => {
            let payload = body::to_bytes(req.into_body()).await?;
            let obj = serde_json::from_slice::<Root>(&payload).unwrap();
            //println!("INFO: received data for {}", obj.name);
            let json_data = serde_json::to_string(&obj).unwrap();
            Ok(Response::new(Body::from(json_data)))
        }
        _ => Ok(Response::new(Body::from(
            "ensure you post to /streamdata endpoint with valid json",
        ))),
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);
    loop {
        let (stream, _) = listener.accept().await?;
        tokio::task::spawn(async move {
            if let Err(err) = Http::new()
                //.http1_only(true)
                .http1_keep_alive(true)
                .serve_connection(stream, service_fn(process_payload))
                .await
            {
                println!("Error serving connection: {:}", err);
            }
        });
    }
}
