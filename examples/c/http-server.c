#define _POSIX_C_SOURCE 200809L

#include <assert.h>
#include <demi/libos.h>
#include <demi/sga.h>
#include <demi/wait.h>
#include <string.h>

#ifdef __linux__
#include <arpa/inet.h>
#include <sys/socket.h>
#endif

#include "common.h"

#define NQTOKEN 1024

int main(int argc, char *const argv[])
{
    struct sockaddr_in saddr = {0};
    int sockqd = -1;
    demi_qtoken_t qts[NQTOKEN] = {-1};
    int nqts = 1;
    demi_qresult_t qr = {0};
    int offset = -1;

    /* Prepare addr */
    saddr.sin_family = AF_INET;
    saddr.sin_port = 9050; 
    assert(inet_pton(AF_INET, "0.0.0.0", &saddr.sin_addr) == 1);

    /* Initialize demikernel */
    assert(demi_init(argc, argv) == 0);

    /* Setup local socket. */
    assert(demi_socket(&sockqd, AF_INET, SOCK_STREAM, 0) == 0);
    assert(demi_bind(sockqd, (const struct sockaddr *)&saddr, sizeof(struct sockaddr_in)) == 0);
    assert(demi_listen(sockqd, 16) == 0);
    assert(demi_accept(&qts[0], sockqd) == 0);

    while (1) {
        assert(demi_wait_any(&qr, &offset, qts, NQTOKEN) == 0);
        if (qr.qr_opcode == DEMI_OPC_ACCEPT) {
            /* Accept */
            fprintf(stdout, "accept\n");
            assert(demi_pop(&qts[nqts++], qr.qr_value.ares.qd) == 0);
        } else if (qr.qr_opcode == DEMI_OPC_POP) {
            /* Pop */
            assert(qr.qr_value.sga.sga_segs != 0);
            fprintf(stdout, "recv: %*s\n",
                    qr.qr_value.sga.sga_segs[0].sgaseg_len,
                    (const char *)qr.qr_value.sga.sga_segs[0].sgaseg_buf);
            /* assert(demi_sgafree(&sga) == 0); */
        } else {
            fprintf(stdout, "opcode: %d", qr.qr_opcode);
        }
    }

    return (EXIT_SUCCESS);
}

