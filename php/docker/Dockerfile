FROM ubuntu:18.04
ARG DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y --force-yes build-essential curl cmake g++ libpcre3-dev libssl-dev make openssl libgmp-dev git

RUN apt-get install -y software-properties-common
RUN LC_ALL=C.UTF-8 add-apt-repository -y ppa:ondrej/php
RUN apt-get update
RUN apt-get install -y php7.1 php7.1-dev php7.1-xml php7.1-mbstring php7.1-curl php7.1-zip php7.1-xdebug

RUN apt-get install -y wget

RUN cd /tmp && wget http://downloads.datastax.com/cpp-driver/ubuntu/16.04/dependencies/libuv/v1.24.0/libuv1_1.24.0-1_amd64.deb && wget http://downloads.datastax.com/cpp-driver/ubuntu/16.04/dependencies/libuv/v1.24.0/libuv1-dev_1.24.0-1_amd64.deb
RUN dpkg -i /tmp/libuv1_1.24.0-1_amd64.deb
RUN dpkg -i /tmp/libuv1-dev_1.24.0-1_amd64.deb


RUN cd /tmp && wget http://downloads.datastax.com/cpp-driver/ubuntu/16.04/cassandra/v2.11.0/cassandra-cpp-driver_2.11.0-1_amd64.deb && wget http://downloads.datastax.com/cpp-driver/ubuntu/16.04/cassandra/v2.11.0/cassandra-cpp-driver-dev_2.11.0-1_amd64.deb
RUN dpkg -i /tmp/cassandra-cpp-driver_2.11.0-1_amd64.deb
RUN dpkg -i /tmp/cassandra-cpp-driver-dev_2.11.0-1_amd64.deb

RUN update-alternatives --set php /usr/bin/php7.1
RUN update-alternatives --set phar /usr/bin/phar7.1
RUN update-alternatives --set phar.phar /usr/bin/phar.phar7.1

RUN pecl install cassandra
RUN echo "extension=cassandra.so" | tee -a /etc/php/7.1/cli/php.ini

RUN curl -sS https://getcomposer.org/installer | php -- \
        --filename=composer \
        --install-dir=/usr/local/bin && \
        echo "alias composer='composer'" >> /root/.bashrc

WORKDIR /var/www

EXPOSE 9000

VOLUME ["/var/www"]

ENTRYPOINT /bin/bash