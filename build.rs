
use std::{
    env,
    fs,
    path::{PathBuf},
    process::Command,
};
use std::path::Path;
use bindgen::{Bindings, CargoCallbacks};
use bindgen::callbacks::ParseCallbacks;
use regex::Regex;

fn download_yosys(yosys_url: &str, yosys_version: &str) -> PathBuf {
    // Compute the tarball URL at runtime.
    let yosys_tarball_url = format!(
        "{yosys_url}/archive/refs/tags/v{yosys_version}.tar.gz"
    );

    // Get the OUT_DIR to download and extract the source.
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    // We will extract the Yosys source into OUT_DIR/yosys-src.
    let yosys_src_dir = out_dir.join("yosys-src");

    // If the Yosys source directory does not exist, download and extract it.
    if !yosys_src_dir.exists() {
        println!("Downloading Yosys {} from {}", yosys_version, yosys_tarball_url);
        // Define the path where the tarball will be saved.
        let tarball_path = out_dir.join(format!("{yosys_version}.tar.gz"));
        // Download the tarball using the "curl" command.
        let status = Command::new("curl")
            .args(&["-L", "-o"])
            .arg(tarball_path.to_str().unwrap())
            .arg(&yosys_tarball_url)
            .status()
            .expect("Failed to execute curl to download Yosys tarball");
        assert!(status.success(), "curl failed to download Yosys tarball: {:?}", status);
        // Extract the tarball using the "tar" command.
        let status = Command::new("tar")
            .args(&["-xzf", tarball_path.to_str().unwrap(), "-C", out_dir.to_str().unwrap()])
            .status()
            .expect("Failed to execute tar command to extract Yosys tarball");
        assert!(status.success(), "tar command failed to extract Yosys tarball: {:?}", status);
        // Rename the extracted folder to "yosys-src"
        let extracted_dir = out_dir.join(format!("yosys-{yosys_version}"));
        fs::rename(&extracted_dir, &yosys_src_dir)
            .expect("Failed to rename extracted Yosys folder");
    } else {
        println!("Using cached Yosys source at {:?}", yosys_src_dir);
    }
    yosys_src_dir
}

fn make_build(yosys_src_dir: &Path) {
    // Define the build directory as OUT_DIR/yosys-build.
    let make_path = find_it("make").unwrap_or_else(|| {
        panic!("make not found in PATH");
    });

    // build command = `make ENABLE_DEBUG=1 ENABLE_LIBYOSYS=1 -j8 libyosys.so`
    let status = Command::new(make_path)
        .current_dir(yosys_src_dir)
        .args(&["ENABLE_DEBUG=1", "ENABLE_LIBYOSYS=1", "-j8", "libyosys.so"])
        .status()
        .expect("Failed to execute make command");
    assert!(status.success(), "make command failed: {:?}", status);

    // find patchelf
    let patchelf_path = find_it("patchelf").unwrap_or_else(|| {
        panic!("patchelf not found in PATH");
    });
    let status = Command::new(patchelf_path)
        .current_dir(yosys_src_dir)
        .args(&["--set-soname", "libyosys.so", "libyosys.so"])
        .status()
        .expect("Failed to execute patchelf command");
    
    assert!(status.success(), "patchelf command failed: {:?}", status);

}

fn find_it<P>(exe_name: P) -> Option<PathBuf>
where P: AsRef<Path>,
{
    let os = env::consts::OS;
    if os != "linux" {
        panic!("Only supports Linux, Unsupported OS: {}", os);
    };

    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths).filter_map(|dir| {
            let full_path = dir.join(&exe_name);
            if full_path.is_file() {
                Some(full_path)
            } else {
                None
            }
        }).next()
    })
}

#[derive(Debug)]
struct MyCallback;
impl ParseCallbacks for MyCallback {
    fn item_name(&self, name: &str) -> Option<String> {
        if name == "Yosys_RTLIL_SigSpec_remove2" {
            return Some("Yosys_RTLIL_SigSpec_remove_2".to_string())
        }
        Some(name.to_string())
    }
}

fn generate_bindings(yosys_build_dir: &Path) {

    // Tell cargo to look for shared libraries in the specified directory
    let manifest_dir_string = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search={}", yosys_build_dir.to_str().unwrap());
    println!("cargo:rustc-link-lib=yosys");

    let mut builder = bindgen::Builder::default()
        .clang_arg("-std=c++20")
        .clang_arg("-D_YOSYS_");

    // Add include paths
    builder = builder
        .clang_arg(format!(
            "-I{}",
            yosys_build_dir.join(".").to_str().unwrap()
        ));

    // Layout Tests Disable
    builder = builder.layout_tests(false);

    // Whitelist
    builder = builder
        .allowlist_type("Yosys::.*");
    builder = builder
        .allowlist_function("Yosys::.*");
    builder = builder
        .allowlist_var("Yosys::.*");
    builder = builder.allowlist_item("Yosys::.*");
    // builder = builder
    //     .allowlist_function("std.*?string.*?");

    // Opaque types
    builder = builder.opaque_type(".*?_Variadic_union.*?");
    builder = builder.opaque_type(".*?collate.*?");
    builder = builder.opaque_type("std::.*?");

    // Needs to be opaque, has generic issues
    builder = builder.opaque_type("Yosys::RTLIL::ObjIterator.*?");
    builder = builder.opaque_type("Yosys::RTLIL::ObjRange.*?");

    // Ignore certain types
    builder = builder.blocklist_type(".*?memory_order.*?");
    builder = builder.blocklist_item("FP_.*");
    builder = builder.blocklist_type(".*?_Bit_iterator");

    // Add the header file
    builder = builder.header(format!("{manifest_dir_string}/include/wrapper.hpp"));




    // Generate the bindings
    let bindings_r = builder
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .parse_callbacks(Box::new(MyCallback {}))
        .generate();

    let bindings: Bindings = match bindings_r {
        Ok(b) => b,
        Err(e) => {
            panic!("Failed to generate bindings: {}", e);
        }
    };

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}


fn main() {
    // Instruct Cargo to re-run this script if build.rs itself changes.
    println!("cargo:rerun-if-changed=build.rs");

    // Define the desired version for Yosys.
    const YOSYS_URL: &str = "https://github.com/YosysHQ/yosys";
    const YOSYS_VERSION: &str = "0.52";

    // Download and extract the Yosys source code.
    let yosys_src_dir = download_yosys(YOSYS_URL, YOSYS_VERSION);

    // Build Yosys using the cmake crate with cxx20.
    make_build(&yosys_src_dir);

    // Generate the Rust bindings for Yosys.
    generate_bindings(&yosys_src_dir);
}