FROM ubuntu:latest

RUN groupadd -r rustroika-group && useradd -r -g rustroika-group rustroika-user

RUN /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
RUN echo >> /home/biadmin/.bashrc
RUN echo 'eval "$(/home/linuxbrew/.linuxbrew/bin/brew shellenv)"' >> /home/biadmin/.bashrc
RUN eval "$(/home/linuxbrew/.linuxbrew/bin/brew shellenv)"

RUN brew install rust

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN cargo fetch

COPY . .
RUN cargo build --release

COPY --from=builder /app/target/release/rustroika .

RUN chmod +x ./docker-entrypoint.sh

USER rustroika-user
ENTRYPOINT [ "./docker-entrypoint.sh" ]