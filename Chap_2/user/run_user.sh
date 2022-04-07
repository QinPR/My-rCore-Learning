cargo build --release
rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/hello_world -O binary target/riscv64gc-unknown-none-elf/release/hello_world.bin
rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/power -O binary target/riscv64gc-unknown-none-elf/release/power.bin
rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/store_fault -O binary target/riscv64gc-unknown-none-elf/release/store_fault.bin
rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/priv_csr -O binary target/riscv64gc-unknown-none-elf/release/priv_csr.bin
rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/priv_inst -O binary target/riscv64gc-unknown-none-elf/release/priv_inst.bin
qemu-riscv64 ./target/riscv64gc-unknown-none-elf/release/hello_world
qemu-riscv64 ./target/riscv64gc-unknown-none-elf/release/power
qemu-riscv64 ./target/riscv64gc-unknown-none-elf/release/priv_csr
qemu-riscv64 ./target/riscv64gc-unknown-none-elf/release/priv_inst
qemu-riscv64 ./target/riscv64gc-unknown-none-elf/release/store_fault
