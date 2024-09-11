FROM rust:latest 

RUN apt-get update && apt-get install -y \
    apt-transport-https \
    ca-certificates \
    curl \
    gnupg \
    lsb-release
RUN curl -fsSL https://download.docker.com/linux/debian/gpg | gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg
RUN echo \
    "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/debian \
    $(lsb_release -cs) stable" | tee /etc/apt/sources.list.d/docker.list > /dev/null
RUN apt-get update && apt-get install -y docker-ce-cli

RUN apt install -y g++-arm-linux-gnueabihf libc6-dev-armhf-cross
RUN rustup target add arm-unknown-linux-gnueabihf
RUN rustup toolchain install stable-arm-unknown-linux-gnueabihf --force-non-host
RUN cargo install cross --git https://github.com/cross-rs/cross

WORKDIR /app 

#CMD ["cargo", "build", "--target", "arm-unknown-linux-gnueabihf"]

ENV CARGO_TARGET_ARM_UNKNOWN_LINUX_GNUEABIHF_LINKER=arm-linux-gnueabihf-gcc CC_arm_unknown_Linux_gnueabihf=arm-linux-gnueabihf-gcc CXX_arm_unknown_linux_gnueabihf=arm-linux-gnueabihf-g++
ENV CROSS_CONTAINER_IN_CONTAINER=true
#CMD ["cargo", "build", "--target", "arm-unknown-linux-gnueabihf"]

#CMD ["cross", "build", "--release", "--target=arm-unknown-linux-gnueabihf"] 
