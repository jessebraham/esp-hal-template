let mcu = variable::get("mcu");

if mcu in ["esp32", "esp32s2", "esp32s3"] {
    // Xtensa devices:
    variable::set("arch",        "xtensa");
    variable::set("toolchain",   "esp");
    variable::set("rust_target", `xtensa-${mcu}-none-elf`);
} else {
    // RISC-V devices:
    let extensions = switch mcu {
        "esp32c2" | "esp32c3" => "imc",
        _ => "imac",
    };

    variable::set("arch",        "riscv");
    variable::set("toolchain",   "stable");
    variable::set("rust_target", `riscv32${extensions}-unknown-none-elf`);
}
