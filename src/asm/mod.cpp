// ----------------------------------
// --- NeuraServer Copyright 2024 ---
// ----------------------------------

extern "C" {
    void server_main() {
        asm(
            ".global server_main\n"
            "server_main:\n"
            "    mov $102, %eax\n"
            "    xor %ebx, %ebx\n"
            "    xor %ecx, %ecx\n"
            "    mov $1, %edx\n"
            "    int $0x80\n"
            "    mov %eax, %ebx\n"
            "    mov $106, %eax\n"
            "    xor %ecx, %ecx\n"
            "    mov $3, %edx\n"
            "    int $0x80\n"
            "    mov %eax, %ebx\n"
            "    mov $106, %eax\n"
            "    mov $10, %ecx\n"
            "    int $0x80\n"
            "accept_loop:\n"
            "    mov %eax, %ebx\n"
            "    mov $102, %eax\n"
            "    xor %ecx, %ecx\n"
            "    xor %edx, %edx\n"
            "    mov $16, %esi\n"
            "    int $0x80\n"
            "    lea server_data, %esi\n"
            "    mov $server_data_end, %edi\n"
            "    sub %esi, %edi\n"
            "    mov %edi, %ecx\n"
            "    call visualize_server_data\n"
            "    mov $4, %eax\n"
            "    mov %eax, %edi\n"
            "    lea message, %esi\n"
            "    mov $12, %edx\n"
            "    int $0x80\n"
            "    mov $6, %eax\n"
            "    int $0x80\n"
            "    jmp accept_loop\n"
        );
    }
}
