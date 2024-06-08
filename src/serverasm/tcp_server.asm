section .data
    ip_addr db '127.0.0.1', 0
    port dw 8080
    msg db 'Hello, client!', 0xA
    msg_len equ $ - msg

section .bss
    server_fd resd 1
    client_fd resd 1
    sockaddr resb 16
    sockaddr_len resd 1
    buffer resb 1024

section .text
    global _start

_start:
    ; Create a socket
    xor rax, rax
    mov al, 41               ; syscall: socket (SYS_socket)
    mov edi, 2               ; AF_INET
    mov esi, 1               ; SOCK_STREAM
    xor edx, edx             ; Protocol (0)
    syscall
    mov [server_fd], eax

    ; Bind the socket
    mov rdi, [server_fd]
    lea rsi, [sockaddr]
    mov dword [rsi], 0x0100007F   ; 127.0.0.1 (IP address)
    mov word [rsi+2], 0x50        ; 8080 (port)
    mov word [rsi+4], 0x2         ; AF_INET (address family)
    xor edx, edx
    mov rax, 49              ; syscall: bind (SYS_bind)
    syscall

    ; Listen for connections
    mov rdi, [server_fd]
    xor esi, esi
    mov al, 50               ; syscall: listen (SYS_listen)
    syscall

accept_loop:
    ; Accept a connection
    mov rdi, [server_fd]
    lea rsi, [sockaddr]
    lea rdx, [sockaddr_len]
    xor r10, r10
    mov al, 43               ; syscall: accept (SYS_accept)
    syscall
    mov [client_fd], eax

    ; Receive data
    mov rdi, [client_fd]
    lea rsi, [buffer]
    mov edx, 1024
    xor r10, r10
    mov al, 45               ; syscall: recv (SYS_recvfrom)
    syscall

    ; Send data back
    mov rdi, [client_fd]
    lea rsi, [msg]
    mov edx, msg_len
    xor r10, r10
    mov al, 44               ; syscall: send (SYS_sendto)
    syscall

    ; Close the client socket
    mov rdi, [client_fd]
    xor rsi, rsi
    mov al, 3                ; syscall: close (SYS_close)
    syscall

    jmp accept_loop          ; Accept the next connection

_exit:
    ; Close the server socket
    mov rdi, [server_fd]
    xor rsi, rsi
    mov al, 3                ; syscall: close (SYS_close)
    syscall

    ; Exit the program
    xor edi, edi
    mov al, 60               ; syscall: exit (SYS_exit)
    syscall
