#pragma once

#include "database.hpp"
#include <boost/beast/http.hpp>
#include <boost/beast/version.hpp>

namespace http = boost::beast::http;

class RequestHandler {
  public:
    RequestHandler(Database db);
    ~RequestHandler();

    http::response<http::string_body>
    handle_request(const http::request<http::string_body>& req);

  private:
    class Impl;
    std::unique_ptr<Impl> pImpl;
};
