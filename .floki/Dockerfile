FROM ubuntu:18.04

# Install rust and cassandra drivers
WORKDIR /tmp
RUN apt-get -qq update && \
    apt-get -y -qq install wget multiarch-support build-essential libssl-dev && \
    wget -O rustup.sh https://sh.rustup.rs && \
    sh rustup.sh -y && \
    wget https://downloads.datastax.com/cpp-driver/ubuntu/18.04/dependencies/libuv/v1.35.0/libuv1_1.35.0-1_amd64.deb \
         https://downloads.datastax.com/cpp-driver/ubuntu/18.04/dependencies/libuv/v1.35.0/libuv1-dev_1.35.0-1_amd64.deb \
         https://downloads.datastax.com/cpp-driver/ubuntu/18.04/cassandra/v2.15.3/cassandra-cpp-driver_2.15.3-1_amd64.deb \
         https://downloads.datastax.com/cpp-driver/ubuntu/18.04/cassandra/v2.15.3/cassandra-cpp-driver-dev_2.15.3-1_amd64.deb && \
    dpkg -i *.deb && \
    rm -f *.deb && \
    apt-get clean
