FROM busybox:latest

COPY ./target/x86_64-unknown-linux-musl/release/q-api-papers /bin/q-api-papers
COPY ./Rocket.toml /root

WORKDIR /root

CMD [ "q-api-papers" ]
