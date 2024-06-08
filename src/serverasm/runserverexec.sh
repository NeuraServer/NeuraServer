# Install NASM
sudo apt-get install nasm

# Run NASM and compile
nasm -f elf64 tcp_server.asm -o tcp_server.o
ld tcp_server.o -o tcp_server

# Run server (With ASM)
./tcp_server

# Run the server (With IP and Telnet)
telnet 127.0.0.1 8080

