use std::path::Path;
use std::path::absolute;

fn main() {
    let arch_board = std::env::var("PAMPA_BOARD").unwrap_or_else(|_| "aarch64/raspi4b".into());
    let (arch, board) = arch_board
        .split_once('/')
        .expect("ERROR: PAMPA_BOARD must be '<arch>/<board>'");

    let boot_path = format!("./src/arch/{arch}/boot.s");
    let boot_file = Path::new(&boot_path);
    if !boot_file.exists() {
        panic!("ERROR: Failed to find boot file for arch {arch}");
    }
    let linker_path = format!("./src/arch/{arch}/{board}/linker.ld");
    let linker_file = Path::new(&linker_path);
    if !linker_file.exists() {
        panic!("ERROR: Failed to find boot file linker file for {arch_board}");
    }

    let absolute_boot_file = absolute(&boot_path).unwrap().to_str().unwrap().to_string();
    let absolute_linker_file = absolute(&linker_path)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    println!("cargo:rustc-link-arg=-T{absolute_linker_file}");
    println!("cargo:rustc-env=PAMPA_BOOT_ASM={absolute_boot_file}");
    println!("cargo:rerun-if-env-changed=PAMPA_BOARD");
    println!("cargo:rerun-if-changed={absolute_linker_file}");
    println!("cargo:rerun-if-changed={absolute_boot_file}");
    println!("cargo:rerun-if-changed=build.rs");
}
