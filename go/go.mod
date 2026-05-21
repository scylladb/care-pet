module github.com/scylladb/care-pet/go

go 1.25.0

require (
	github.com/codahale/hdrhistogram v0.0.0-20161010025455-3a0bb77429bd
	github.com/go-openapi/errors v0.19.6
	github.com/go-openapi/loads v0.19.5
	github.com/go-openapi/runtime v0.19.20
	github.com/go-openapi/spec v0.19.9
	github.com/go-openapi/strfmt v0.19.5
	github.com/go-openapi/swag v0.19.9
	github.com/go-openapi/validate v0.19.10
	github.com/gocql/gocql v0.0.0-20211015133455-b225f9b53fa1
	github.com/jessevdk/go-flags v1.4.0
	github.com/scylladb/gocqlx/v2 v2.8.0
	github.com/spf13/pflag v1.0.5
	golang.org/x/net v0.53.0
)

require (
	github.com/PuerkitoBio/purell v1.1.1 // indirect
	github.com/PuerkitoBio/urlesc v0.0.0-20170810143723-de5bf2ad4578 // indirect
	github.com/asaskevich/govalidator v0.0.0-20200428143746-21a406dcc535 // indirect
	github.com/docker/go-units v0.4.0 // indirect
	github.com/go-openapi/analysis v0.19.10 // indirect
	github.com/go-openapi/jsonpointer v0.19.3 // indirect
	github.com/go-openapi/jsonreference v0.19.4 // indirect
	github.com/go-stack/stack v1.8.0 // indirect
	github.com/google/uuid v1.6.0 // indirect
	github.com/klauspost/compress v1.18.5 // indirect
	github.com/mailru/easyjson v0.7.1 // indirect
	github.com/mitchellh/mapstructure v1.3.2 // indirect
	github.com/scylladb/go-reflectx v1.0.1 // indirect
	go.mongodb.org/mongo-driver v1.3.4 // indirect
	golang.org/x/sync v0.20.0 // indirect
	golang.org/x/text v0.36.0 // indirect
	gopkg.in/inf.v0 v0.9.1 // indirect
	gopkg.in/yaml.v2 v2.3.0 // indirect
)

replace github.com/gocql/gocql => github.com/scylladb/gocql v1.18.0
