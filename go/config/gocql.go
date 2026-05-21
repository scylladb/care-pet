package config

import (
	"time"

	"github.com/gocql/gocql"
	"github.com/spf13/pflag"

	"github.com/scylladb/care-pet/go/db"
	"github.com/scylladb/gocqlx/v2"
)

var config = struct {
	Hosts    []string
	Timeout  time.Duration
	DialTimeout time.Duration
	Password gocql.PasswordAuthenticator
}{}

func init() {
	pflag.StringArrayVar(&config.Hosts, "hosts", []string{"127.0.0.1"}, "cluster nodes address list")
	pflag.DurationVar(&config.Timeout, "timeout", 60*time.Second, "connection timeout")
	pflag.DurationVar(&config.DialTimeout, "dial-timeout", 5*time.Second, "initial dial timeout")

	pflag.StringVar(&config.Password.Username, "username", "", "password based authentication username")
	pflag.StringVar(&config.Password.Password, "password", "", "password based authentication password")
}

func Config() gocql.ClusterConfig {
	cluster := gocql.NewCluster(config.Hosts...)

	cluster.Consistency = gocql.LocalOne
	cluster.PoolConfig.HostSelectionPolicy = gocql.TokenAwareHostPolicy(gocql.RoundRobinHostPolicy())
	cluster.Timeout = config.Timeout
	cluster.ConnectTimeout = config.DialTimeout

	if config.Password.Username != "" {
		cluster.Authenticator = config.Password
	}

	return *cluster
}

// Session returns new session
func Session() (*gocql.Session, error) {
	return gocql.NewSession(Config())
}

// Keyspace returns new session with specified keyspace
func Keyspace() (gocqlx.Session, error) {
	cfg := Config()
	cfg.Keyspace = db.KeySpace
	return gocqlx.WrapSession(gocql.NewSession(cfg))
}
