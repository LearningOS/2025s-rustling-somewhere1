//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // In tests7, we should set up an environment variable
    // called `TEST_FOO`. Print in the standard output to let
    // Cargo do it.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs(); // What's the use of this timestamp here?
    // 构建设置环境变量的命令
    let set_env_command = format!("rustc-env=TEST_FOO={}", timestamp);
    println!("cargo:{}", set_env_command); // 输出命令，让 Cargo 设置环境变量

    // 对于 tests8 的部分，你需要启用 "pass" 特性
    // 这通常是通过设置 cargo:rustc-cfg 来实现的
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
