# `bevy_no_std_targets`

Collection of test-crates for various `no_std` platforms.

| Platform | Crate | Compiles | Runs |
| - | - | - | - |
| UEFI | [platform_uefi](crates/platform_uefi/) | Yes | Yes |
| GameBoy Advance | [platform_gba](crates/platform_gba/) | Blocked on Atomic CAS | - |
| Nintendo Switch | [platform_switch](crates/platform_switch/) | Yes | Untested |
| PlayStation One | [platform_psx](crates/platform_psx/) | Blocked on Atomic CAS | - |
| MSP430 | [platform_msp430](crates/platform_msp430/) | No (16-bit not supported) | - |
