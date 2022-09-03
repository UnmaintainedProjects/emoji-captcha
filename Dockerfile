FROM rust AS build
WORKDIR /app
COPY src src
COPY Cargo.toml .
RUN cargo install --path .

FROM debian
COPY --from=build /usr/local/cargo/bin/emoji-captcha /usr/local/bin
COPY emoji-data-ios emoji-data-ios
COPY allowed-emojis.txt .
RUN ls /usr/local/bin
CMD ["/usr/local/bin/emoji-captcha"]
