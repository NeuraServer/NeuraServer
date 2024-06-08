#include <iostream>
#include <thread>
#include <chrono>

void monitorServer() {
    while (true) {
        // Simulate checking server status
        std::cout << "Checking server status..." << std::endl;
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
