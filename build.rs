fn main() {
    // 编译Slint UI文件
    let config = slint_build::CompilerConfiguration::new()
        .with_style("fluent".into())
        .with_include_paths(vec!["ui".into()]);
    
    slint_build::compile_with_config("ui/main.slint", config).unwrap();
    
    // 告诉cargo在UI文件改变时重新运行构建脚本
    println!("cargo:rerun-if-changed=ui/");
    println!("cargo:rerun-if-changed=ui/globals/");
    println!("cargo:rerun-if-changed=ui/components/");
    println!("cargo:rerun-if-changed=ui/pages/");
    println!("cargo:rerun-if-changed=build.rs");
}
