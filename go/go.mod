module github.com/scylladb/care-pet/go

go 1.14

require (
	github.com/HdrHistogram/hdrhistogram-go v1.1.2
	github.com/go-openapi/errors v0.22.0
	github.com/go-openapi/loads v0.22.0
	github.com/go-openapi/runtime v0.28.0
	github.com/go-openapi/spec v0.21.0
	github.com/go-openapi/strfmt v0.23.0
	github.com/go-openapi/swag v0.23.0
	github.com/go-openapi/validate v0.24.0
	github.com/gocql/gocql v1.6.0
	github.com/jessevdk/go-flags v1.4.0
	github.com/scylladb/gocqlx/v2 v2.8.0
	github.com/spf13/pflag v1.0.5
	golang.org/x/net v0.25.0
)

replace github.com/gocql/gocql => github.com/scylladb/gocql v1.14.0
