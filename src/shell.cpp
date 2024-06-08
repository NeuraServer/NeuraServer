#include <iostream>
#include <string>
#include <cstdlib>
#include <cstdio>
#include <memory>
#include <stdexcept>
#include <array>
#include <thread>
#include <chrono>

void print_help() {
    std::cout << "NeuraShell Commands:\n";
    std::cout << "help          - Show this help message\n";
    std::cout << "start_server  - Start the server\n";
    std::cout << "stop_server   - Stop the server\n";
    std::cout << "status        - Check if the server is running\n";
    std::cout << "exit          - Exit NeuraShell\n";
}

bool is_server_running() {
    std::array<char, 128> buffer;
    std::string result;
    std::unique_ptr<FILE, decltype(&pclose)> pipe(popen("pgrep neuraserver", "r"), pclose);
    if (!pipe) {
        throw std::runtime_error("popen() failed!");
    }
    while (fgets(buffer.data(), buffer.size(), pipe.get()) != nullptr) {
        result += buffer.data();
    }
    return !result.empty();
}

void start_server() {
    if (is_server_running()) {
        std::cout << "Server is already running.\n";
        return;
    }
    std::cout << "Starting server...\n";
    int result = std::system("cargo run --manifest-path main.rs &");
    if (result != 0) {
        std::cerr << "Failed to start the server.\n";
    } else {
        std::this_thread::sleep_for(std::chrono::seconds(2));
        if (is_server_running()) {
            std::cout << "Server started successfully.\n";
        } else {
            std::cerr << "Failed to start the server.\n";
        }
    }
}

void stop_server() {
    if (!is_server_running()) {
        std::cout << "Server is not running.\n";
        return;
    }
    std::cout << "Stopping server...\n";
    int result = std::system("pkill neuraserver");
    if (result != 0) {
        std::cerr << "Failed to stop the server or server not running.\n";
    } else {
        std::this_thread::sleep_for(std::chrono::seconds(2));
        if (!is_server_running()) {
            std::cout << "Server stopped successfully.\n";
        } else {
            std::cerr << "Failed to stop the server.\n";
        }
    }
}

void check_status() {
    if (is_server_running()) {
        std::cout << "Server is running.\n";
    } else {
        std::cout << "Server is not running.\n";
    }
}

int main() {
    std::string command;

    std::cout << "Welcome to NeuraServer Shell! Type 'help' to see available commands.\n";

    while (true) {
        std::cout << "nsshell/:> ";
        std::getline(std::cin, command);

        if (command == "help") {
            print_help();
        } else if (command == "start_server") {
            start_server();
        } else if (command == "stop_server") {
            stop_server();
        } else if (command == "status") {
            check_status();
        } else if (command == "exit") {
            std::cout << "Exiting NeuraShell...\n";
            break;
        } else {
            std::cout << "Unknown command: " << command << "\n";
            std::cout << "Type 'help' to see available commands.\n";
        }
    }

    return 0;
}
