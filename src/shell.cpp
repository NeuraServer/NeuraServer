#include <iostream>
#include <fstream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>
#include <unistd.h>
#include <arpa/inet.h>
#include <thread>
#include <chrono>
#include <atomic>

class ServerConnection {
public:
    ServerConnection(const std::string& ip, int port) : ip(ip), port(port), sock(-1) {
        connectToServer();
    }

    void connectToServer() {
        if ((sock = socket(AF_INET, SOCK_STREAM, 0)) < 0) {
            throw std::runtime_error("Socket creation error");
        }

        server_address.sin_family = AF_INET;
        server_address.sin_port = htons(port);

        if (inet_pton(AF_INET, ip.c_str(), &server_address.sin_addr) <= 0) {
            throw std::runtime_error("Invalid address or address not supported");
        }

        if (connect(sock, (struct sockaddr*)&server_address, sizeof(server_address)) < 0) {
            throw std::runtime_error("Connection failed");
        }
    }

    void sendData(const std::string& data) {
        send(sock, data.c_str(), data.length(), 0);
    }

    std::string receiveData() {
        char buffer[1024] = {0};
        read(sock, buffer, 1024);
        return std::string(buffer);
    }

    ~ServerConnection() {
        close(sock);
    }

private:
    std::string ip;
    int port;
    int sock;
    struct sockaddr_in server_address;
};

std::vector<std::string> split(const std::string& str, char delimiter) {
    std::vector<std::string> tokens;
    std::string token;
    std::istringstream tokenStream(str);
    while (std::getline(tokenStream, token, delimiter)) {
        tokens.push_back(token);
    }
    return tokens;
}

void executeCommand(const std::string& command) {
    std::system(command.c_str());
}

void liveMonitor(ServerConnection& server, std::atomic<bool>& running) {
    while (running.load()) {
        server.sendData("status");
        std::string response = server.receiveData();
        std::cout << "Server Status: " << response << std::endl;
        std::this_thread::sleep_for(std::chrono::seconds(5));
    }
}

void startLiveMonitoring(ServerConnection& server, std::atomic<bool>& running) {
    std::thread monitoringThread(liveMonitor, std::ref(server), std::ref(running));
    monitoringThread.detach();
}

void checkServerHealth(ServerConnection& server) {
    server.sendData("health_check");
    std::string response = server.receiveData();
    std::cout << "Server Health: " << response << std::endl;
}

int main() {
    std::ifstream config_file("config.txt");
    if (!config_file.is_open()) {
        std::cerr << "Could not open config file." << std::endl;
        return 1;
    }

    std::string line;
    std::string ip;
    int port;
    while (std::getline(config_file, line)) {
        auto tokens = split(line, '=');
        if (tokens.size() != 2) {
            continue;
        }
        if (tokens[0] == "IP") {
            ip = tokens[1];
        } else if (tokens[0] == "PORT") {
            port = std::stoi(tokens[1]);
        }
    }

    try {
        ServerConnection server(ip, port);
        std::atomic<bool> running(true);

        startLiveMonitoring(server, running);

        std::string command;
        while (true) {
            std::cout << "Enter command: ";
            std::getline(std::cin, command);
            if (command == "exit") {
                running.store(false);
                break;
            } else if (command == "health") {
                checkServerHealth(server);
            } else {
                server.sendData(command);
                std::string response = server.receiveData();
                std::cout << "Response: " << response << std::endl;

                executeCommand(command);
            }
        }

    } catch (const std::exception& ex) {
        std::cerr << "Error: " << ex.what() << std::endl;
    }

    return 0;
}
