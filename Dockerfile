# Rust as the base build image
FROM rust:1.66 as build

# 1. Create a new empty shell project
RUN USER=root cargo new --bin balance
WORKDIR /balance

# 2. Copy our manifests
COPY ./Cargo.lock ./Cargo.lock 
COPY ./Cargo.toml ./Cargo.toml 

# 3. Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# 4. Now that the dependency is built, copy your source code
COPY ./src ./src
# COPY ./defaults ./defaults
COPY balance.yml balance.yml
COPY .env .env

# 5. Build for release.
RUN rm ./target/release/deps/balance*
RUN cargo build --release
RUN rm .env
# RUN rm README.md

# # 6. Dockerfile3: Our final base (1.26GB)
# # FROM rust:1.61
# # 6. Dockerfile4: space-saving image variant (675MB)
# # FROM rust:1.61-slim-buster

# 6. Dockerfile5: Linux image without any rust (75.9MB)
FROM debian:buster-slim

# 7. Copy the build artifact from the build stage
COPY --from=build /balance/target/release/balance .
COPY --from=build /balance/balance.yml .
# COPY --from=build /balance/.env .
# Update debian to enable vim for editing configuration files
# RUN apt update && apt install vim -y
# RUN apt update && apt install vim make wget -y
#
# SEE https://stackoverflow.com/questions/66484445/version-glibc-2-29-not-found
#
# wget -c https://ftp.gnu.org/gnu/glibc/glibc-2.29.tar.gz
# tar -zxvf glibc-2.29.tar.gz
# cd glibc-2.29
# ./configure --prefix=/opt/glibc
# make 
# make install

# 8. Set the startup command to run our binary
ENTRYPOINT ./balance

# CMD ["./balance"]
# # https://stackoverflow.com/a/53897608 | How to pass arguments to Shell Script through docker run
# # RUN chmod 755 /balance/target/release/balance
# # https://stackoverflow.com/questions/31523551/how-can-i-pass-arguments-to-a-docker-container-with-a-python-entry-point-script
# #
# # exec form 
# # ENTRYPOINT ["./balance"]
# # shell form


