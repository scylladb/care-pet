FROM golang:1.14

ENV container docker

RUN git clone https://github.com/scylladb/care-pet /care-pet

RUN cd /care-pet/go && \
    go build ./cmd/migrate && \
    go build ./cmd/sensor && \
    go build ./cmd/loadtest && \
    go build ./cmd/server

ENV PATH /care-pet/go:$PATH

EXPOSE 8000

