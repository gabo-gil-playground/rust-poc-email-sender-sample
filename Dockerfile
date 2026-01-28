# based on rust official image:
# https://www.docker.com/blog/simplify-your-deployments-using-the-rust-official-image/
#
# Example:
# - docker build -t rust-poc-email-sender-sample:latest --ssh default=[FULL_PATH_AND_SSH_RUST_CRATES_PRIVATE_KEY] .
# - docker run -p 0.0.0.0:9026:9026 --env-file ./docker/dev-environment.env rust-poc-email-sender-sample:latest
#
FROM rust:latest as builder
RUN mkdir -p /log/config/
ADD logging_config.yaml /log/config/logging_config.yaml

WORKDIR /src
COPY . .

# install ssh client and set SSH for git repositories
RUN apt-get install openssh-client -y
RUN --mount=type=ssh mkdir -p -m 0600 ~/.ssh && ssh-keyscan github.com >> ~/.ssh/known_hosts
RUN git config --global url."git@github.com:".insteadOf "https://github.com/"

RUN --mount=type=ssh cargo install --path .

FROM debian:latest
RUN apt-get update && apt-get install -y --no-install-recommends build-essential libpq-dev ca-certificates
RUN apt-get install -y extra-runtime-dependencies & rm -rf /var/lib/apt/lists/*
RUN update-ca-certificates

COPY --from=builder /log/config/logging_config.yaml /log/config/logging_config.yaml
COPY --from=builder /usr/local/cargo/bin/rust-poc-email-sender-sample /usr/local/bin/rust-poc-email-sender-sample
CMD ["rust-poc-email-sender-sample"]
