FROM ubuntu:22.04 AS builder

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y \
    cmake g++ git libssl-dev libuv1-dev zlib1g-dev \
    && rm -rf /var/lib/apt/lists/*

# Build ScyllaDB C++ driver from source
WORKDIR /deps
RUN git clone --depth 1 --branch master https://github.com/scylladb/cpp-driver.git && \
    cmake -B cpp-driver/build -S cpp-driver \
        -DCMAKE_BUILD_TYPE=Release \
        -DCMAKE_INSTALL_PREFIX=/usr/local \
        -DCASS_BUILD_SHARED=ON \
        -DCASS_BUILD_STATIC=OFF && \
    cmake --build cpp-driver/build --parallel && \
    cmake --install cpp-driver/build && \
    ldconfig

WORKDIR /app
COPY . .
RUN cmake -B build -DCMAKE_BUILD_TYPE=Release && \
    cmake --build build --parallel

# ---------------------------------------------------------------------------
FROM ubuntu:22.04

RUN apt-get update && apt-get install -y libuv1 libssl3 && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/lib/x86_64-linux-gnu/libscylla-cpp-driver.so* /usr/local/lib/x86_64-linux-gnu/
COPY --from=builder /app/build/migrate \
                    /app/build/sensor  \
                    /app/build/server  \
                    /usr/local/bin/
RUN ldconfig

CMD ["server"]
