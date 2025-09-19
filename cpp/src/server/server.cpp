//
// Copyright (c) 2016-2019 Vinnie Falco (vinnie.falco@gmail.com)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//
// Official repository: https://github.com/boostorg/beast
//

//------------------------------------------------------------------------------
//
// Example: HTTP server, asynchronous
//
//------------------------------------------------------------------------------

#include <boost/asio/dispatch.hpp>
#include <boost/asio/strand.hpp>
#include <boost/beast/core.hpp>
#include <boost/beast/http.hpp>
#include <boost/beast/http/string_body.hpp>
#include <boost/beast/version.hpp>
#include <boost/config.hpp>
#include <cassandra.h>
#include <iostream>
#include <string>
#include <thread>

#include "database.hpp"
#include "handlers.hpp"

namespace beast = boost::beast;
namespace http = beast::http;
namespace net = boost::asio;
using tcp = boost::asio::ip::tcp;

// Report a failure
void fail(beast::error_code ec, char const* what) {
    std::cerr << what << ": " << ec.message() << "\n";
}

// Helper function to send an HTTP message
template <class Stream, bool isRequest, class Body, class Fields>
void send_message(Stream& stream, bool& close, beast::error_code& ec,
                  http::message<isRequest, Body, Fields>&& msg) {
    // Determine if we should close the connection after
    close = msg.need_eof();

    // We need the serializer here because the serializer requires
    // a non-const file_body, and the message oriented version of
    // http::write only works with const messages.
    http::serializer<isRequest, Body, Fields> sr{msg};
    http::write(stream, sr, ec);
}

// Handles an HTTP server connection
void do_session(beast::tcp_stream& stream, RequestHandler& rh) {
    bool close = false;
    beast::error_code ec;

    // This buffer is required to persist across reads
    beast::flat_buffer buffer;

    for (;;) {
        // Read a request
        http::request<http::string_body> req;
        http::read(stream, buffer, req, ec);
        if (ec == http::error::end_of_stream) {
            break;
        }

        if (ec) {
            return fail(ec, "read");
        }

        // Send the response
        http::response<http::string_body> response;
        try {
            response = rh.handle_request(req);
        } catch (std::exception const& e) {
            http::response<http::string_body> res{
                http::status::internal_server_error, req.version()};
            res.set(http::field::server, BOOST_BEAST_VERSION_STRING);
            res.set(http::field::content_type, "text/html");
            res.keep_alive(req.keep_alive());
            res.body() = std::format(
                "Internal error (unhandled exception thrown): {}", e.what());
            res.prepare_payload();
            response = res;
        } catch (...) {
            http::response<http::string_body> res{
                http::status::internal_server_error, req.version()};
            res.set(http::field::server, BOOST_BEAST_VERSION_STRING);
            res.set(http::field::content_type, "text/html");
            res.keep_alive(req.keep_alive());
            res.body() = "Internal server error - unknown unhandled exception ";
            res.prepare_payload();
            response = res;
        }

        // Send the response using the helper function
        send_message(stream, close, ec, std::move(response));

        if (ec) {
            return fail(ec, "write");
        }

        if (close) {
            // This means we should close the connection, usually because
            // the response indicated the "Connection: close" semantic.
            break;
        }
    }

    // Send a TCP shutdown
    beast::error_code ignored =
        stream.socket().shutdown(tcp::socket::shutdown_send, ec);

    // At this point the connection is closed gracefully
}

// Accepts incoming connections and launches the sessions
void do_listen(net::io_context& ioc, tcp::endpoint endpoint,
               RequestHandler& rh) {
    beast::error_code ec;

    // Open the acceptor
    tcp::acceptor acceptor(ioc);

    if (acceptor.open(endpoint.protocol(), ec)) {
        return fail(ec, "open");
    }

    // Allow address reuse
    if (acceptor.set_option(net::socket_base::reuse_address(true), ec)) {
        return fail(ec, "set_option");
    }

    // Bind to the server address
    if (acceptor.bind(endpoint, ec)) {
        return fail(ec, "bind");
    }

    // Start listening for connections
    if (acceptor.listen(net::socket_base::max_listen_connections, ec)) {
        return fail(ec, "listen");
    }

    for (;;) {
        tcp::socket socket{ioc};
        if (acceptor.accept(socket, ec)) {
            fail(ec, "accept");
            continue;
        }

        // Launch the session, transferring ownership of the socket
        std::thread{[](beast::tcp_stream stream, RequestHandler& rh) {
                        do_session(stream, rh);
                    },
                    beast::tcp_stream(std::move(socket)), std::ref(rh)}
            .detach();
    }
}

void run_server(const boost::program_options::variables_map& vm) {
    auto const address = net::ip::make_address(vm["host"].as<std::string>());
    auto const port = vm["port"].as<unsigned short>();

    Database db(vm);
    RequestHandler rh(std::move(db));

    // The io_context is required for all I/O
    net::io_context ioc{};

    // Create and launch a listening port
    std::cout << "Server listening on " << address << ":" << port << std::endl;
    do_listen(ioc, tcp::endpoint{address, port}, rh);
}
