#include <iostream>
#include <thread>
#include <chrono>
#include <boost/asio.hpp>

using boost::asio::ip::tcp;

void monitorServer() {
    boost::asio::io_context io_context;
    tcp::resolver resolver(io_context);
    tcp::resolver::results_type endpoints = resolver.resolve("127.0.0.1", "5500");

    while (true) {
        try {
            tcp::socket socket(io_context);
            boost::asio::connect(socket, endpoints);

            // Send health check request
            std::string request = "GET /health_check HTTP/1.1\r\nHost: 127.0.0.1:5500\r\n\r\n";
            boost::asio::write(socket, boost::asio::buffer(request));

            // Read response
            boost::asio::streambuf response;
            boost::asio::read_until(socket, response, "\r\n");

            std::istream response_stream(&response);
            std::string http_version;
            response_stream >> http_version;
            unsigned int status_code;
            response_stream >> status_code;
            std::string status_message;
            std::getline(response_stream, status_message);

            if (status_code == 200) {
                std::cout << "Server is healthy." << std::endl;
            } else {
                std::cout << "Server is unhealthy. Status code: " << status_code << std::endl;
            }
        } catch (std::exception& e) {
            std::cerr << "Error: " << e.what() << std::endl;
        }

        std::this_thread::sleep_for(std::chrono::seconds(10));
    }
}

int main() {
    std::thread monitorThread(monitorServer);
    monitorThread.detach();

    std::cout << "Server monitor started." << std::endl;

    // Simulate server running
    while (true) {
        std::this_thread::sleep_for(std::chrono::seconds(60));
    }

    return 0;
}
