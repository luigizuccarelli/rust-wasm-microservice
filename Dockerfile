FROM scratch
COPY ./target//release/rust-wasm-microservice /microservice
EXPOSE 8080
ENTRYPOINT [ "microservice" ]
