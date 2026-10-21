# Change the PHP version here. Use a PHP 8.4 release that has all security fixes.
ARG PHP_VERSION=8.4.26
ARG SCYLLADB_PHP_DRIVER_VERSION=v1.5.1

# Build the ScyllaDB PHP driver (cassandra.so) from source for this PHP version.
# The build links the ScyllaDB C/C++ driver statically into the extension.
FROM php:${PHP_VERSION}-cli AS driver
ARG SCYLLADB_PHP_DRIVER_VERSION
RUN apt-get update && apt-get install -y --no-install-recommends \
        build-essential cmake ninja-build pkg-config git ca-certificates \
        libuv1-dev libssl-dev libgmp-dev zlib1g-dev \
    && rm -rf /var/lib/apt/lists/*
RUN git clone --depth 1 --branch "${SCYLLADB_PHP_DRIVER_VERSION}" \
        https://github.com/he4rt/scylladb-php-driver.git /src/scylladb-php-driver
WORKDIR /src/scylladb-php-driver
RUN cmake -G Ninja -B build \
        -DCUSTOM_PHP_CONFIG="$(command -v php-config)" \
        -DCMAKE_BUILD_TYPE=Release \
        -DPHP_DEBUG=OFF \
        -DPHP_THREAD_SAFE=OFF \
        -DPHP_SCYLLADB_DRIVER_VERSION="${SCYLLADB_PHP_DRIVER_VERSION#v}" \
    && cmake --build build --parallel "$(nproc)" \
    && cmake --install build --component extension

FROM php:${PHP_VERSION}-cli
RUN apt-get update && apt-get install -y --no-install-recommends \
        libuv1t64 libgmp10 git unzip \
    && rm -rf /var/lib/apt/lists/*
COPY --from=driver /usr/local/lib/php/extensions/ /usr/local/lib/php/extensions/
RUN docker-php-ext-enable cassandra && php -m | grep -qx cassandra
COPY --from=composer:2 /usr/bin/composer /usr/local/bin/composer

WORKDIR /var/www

EXPOSE 8000

CMD ["sleep", "infinity"]
