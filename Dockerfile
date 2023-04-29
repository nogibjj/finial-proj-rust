# From rust:latest as builder
# ENV APP rust-bert-rocket
# WORKDIR /usr/src/$APP
# COPY . .
# RUN cargo install --path .

# FROM debian:buster-slim
# RUN apt-get update &&\
#     rm -rf ~/.cache &&\
#     apt clean all &&\
#     apt install -y cmake &&\
#     apt install -y clang &&\
#     apt install -y wget &&\
#     apt install -y unzip
# RUN wget https://download.pytorch.org/libtorch/cpu/libtorch-cxx11-abi-shared-with-deps-1.13.1%2Bcpu.zip -O libtorch.zip
# RUN unzip -o libtorch.zip
# ENV LIBTORCH /usr/src/$APP/libtorch 
# ENV LD_LIBRARY_PATH /usr/src/$APP/libtorch/lib:$LD_LIBRARY_PATH
# COPY --from=builder /usr/local/cargo/bin/$APP /usr/local/bin/$APP
# ENTRYPOINT ["/usr/local/bin/rust-bert-rocket"]
# #export this actix web service to port 8080 and 0.0.0.0
# EXPOSE 8080
# CMD ["rust-bert-rocket"]

FROM rust:latest
ENV APP rust-bert-rocket
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000
WORKDIR /usr/src/$APP
COPY . .
RUN apt-get update && rm -rf /var/lib/apt/lists/*
RUN rustup default nightly
RUN cargo install --path .
RUN cargo build -j 16
EXPOSE 8000
CMD ["cargo", "run"]