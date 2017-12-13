// cc src/bin/client.c -o client (pkg-config --cflags libmodbus)

#include <errno.h>
#include <modbus.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(int arc, char **argv) {
  printf("Modbus C Client\n");

  modbus_t *ctx = NULL;

  ctx = modbus_new_rtu("/dev/ttyUSB0", 9600, 'N', 8, 1);

  if (ctx == NULL) {
      fprintf(stderr, "Unable to allocate libmodbus context\n");
      return -1;
  }
  modbus_set_debug(ctx, TRUE);
  modbus_set_error_recovery(ctx,
                            MODBUS_ERROR_RECOVERY_LINK |
                            MODBUS_ERROR_RECOVERY_PROTOCOL);

  modbus_set_slave(ctx, 35);

  if (modbus_connect(ctx) == -1) {
    fprintf(stderr, "Connection failed: %s\n", modbus_strerror(errno));
    modbus_free(ctx);
    return -1;
  }

  int serial_mode = modbus_rtu_get_serial_mode(ctx);
  printf("%d\n", serial_mode);

  // if (modbus_rtu_set_serial_mode(ctx, MODBUS_RTU_RS485) == -1) {
  //   fprintf(stderr, "Could not set serial mode: %s\n", modbus_strerror(errno));
  //   modbus_free(ctx);
  //   return -1;
  // }

  uint8_t *dest = NULL;

  dest = (uint8_t *) malloc(10 * sizeof(uint8_t));
  memset(dest, 0, 10 * sizeof(uint8_t));

  if (modbus_read_bits(ctx, 0, 4, dest) == -1) {
    fprintf(stderr, "Could not read bit: %s\n", modbus_strerror(errno));
    modbus_free(ctx);
    return -1;
  }

  for (int i = 0; i < 11; i++) {
    printf("%d ", dest[i]);
  }
  printf("\n");

  return 0;
}
