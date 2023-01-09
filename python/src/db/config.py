import argparse


def argument_parser():
    parser = argparse.ArgumentParser(add_help=False)
    parser.add_argument("-h", "--hosts",
                        help="ScyllaDB node address",
                        required=True, default="127.0.0.1")
    parser.add_argument("-u", "--username",
                        help="Password based authentication username")
    parser.add_argument("-p", "--password",
                        help="Password based authentication password")
    parser.add_argument("-d", "--datacenter",
                        help="Local data center")
    return parser
