.global server_main
server_main:
mov $102, %eax
xor %ebx, %ebx
xor %ecx, %ecx
mov $1, %edx
int $0x80
mov %eax, %ebx
mov $106, %eax
xor %ecx, %ecx
mov $3, %edx
int $0x80
mov %eax, %ebx
mov $106, %eax
mov $10, %ecx
int $0x80
accept_loop:
mov %eax, %ebx
mov $102, %eax
xor %ecx, %ecx
xor %edx, %edx
mov $16, %esi
int $0x80
lea server_data, %esi
mov $server_data_end, %edi
sub %esi, %edi
mov %edi, %ecx
call visualize_server_data
mov $4, %eax
mov %eax, %edi
lea message, %esi
mov $12, %edx
int $0x80
mov $6, %eax
int $0x80
jmp accept_loop
