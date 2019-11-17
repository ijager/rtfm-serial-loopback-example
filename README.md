# RTFM-rs Serial loopback for stm32f103 (black pill)

## Pins

| Function | Pin |
|---|---|
| UART2 Tx | PA2 |
| UART2 Rx | PA3 |
| UART3 Tx | PB10 |
| UART3 Rx | PB11 |

## Connections

UART3_Tx <-> UART2_Rx

UART2_Tx -> serial_adapter -> computer

## Build and flash

```console
bobbin load --bin rtfm-serial-loopback-example
```