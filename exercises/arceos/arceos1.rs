// arceos1.rs
//
// Execute `rustlings hint arceos1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE
use std::fs;
use std::process::{Command};

// #[cfg(target_arch = "x86_64")]
// let qemu = "qemu-system-x86_64";

// #[cfg(target_arch = "aarch64")]
// let qemu = "qemu-system-aarch64";

// #[cfg(target_arch = "riscv64")]
// let qemu = "qemu-system-riscv64";
// test

fn d1() -> String {

    let directory_path = "exercises/arceos";
    let mut result = String::from("");
    // 使用 read_dir() 函数获取目录中的条目列表
    if let Ok(entries) = fs::read_dir(directory_path) {
        println!("{:#?}", entries);
        for entry in entries {
            if let Ok(entry) = entry {
                let file_path = entry.path();
                let file_name = file_path.to_string_lossy(); // 转换为字符串
                println!("File: {}", file_name);
                if file_name.contains(".bin") {
                    // let bin_path = "/Users/jiangkun/codes/rust/arceos/apps/helloworld/helloworld_qemu-virt-aarch64.bin";
                    let bin_path = file_name.clone(); // format!("exercises/arceos/{}", file_name);
                    let archext: Vec<&str> = file_name.split('-').collect();
                    let arch_:Vec<&str> = archext.get(archext.len()-1).unwrap().split('.').collect();
                    let arch = arch_.get(0).unwrap();
                    let qemu = format!("qemu-system-{}", arch);

                    let sh = format!("{} -m 128M -smp 4 -cpu cortex-a72 -machine virt -kernel {} -nographic", qemu, bin_path);

                    println!("脚本：{}", sh);

                    let res = Command::new("sh")
                    .arg("-c")
                    .arg(sh)
                    .output().unwrap().stdout;
                    result = String::from_utf8(res).unwrap();
                    eprintln!("{}", result);
                }
            }
        }
    } else {
        eprintln!("Error reading directory.");
    }
    result

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let strs = d1();
        let mut result = "";
        if strs.contains("Hello, world!") {
            result = "Hello, world!";
        } else {
            println!("error");
        }
        assert_eq!(result, "Hello, world!");
    }
}
