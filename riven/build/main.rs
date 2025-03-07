// `cargo:rustc-check-cfg` doesn't silence warnings within the build script.
#![allow(unexpected_cfgs)]

#[cfg(riven_autogen)]
pub mod autogen;

pub fn main() {
    const RIVEN_AUTOGEN_NONCE: &str = "RIVEN_AUTOGEN_NONCE";
    const RIVEN_AUTOGEN_DEVMODE: &str = "RIVEN_AUTOGEN_DEVMODE";
    const CFG_RIVEN_AUTOGEN: &str = "riven_autogen";
    const CFG_RIVEN_AUTOGEN_OUTDIR: &str = "riven_autogen_outdir";

    println!("cargo:rerun-if-changed=build");
    println!("cargo:rerun-if-env-changed={}", RIVEN_AUTOGEN_NONCE);
    println!("cargo:rerun-if-env-changed={}", RIVEN_AUTOGEN_DEVMODE);
    println!("cargo:rustc-check-cfg=cfg({})", CFG_RIVEN_AUTOGEN);
    println!("cargo:rustc-check-cfg=cfg({})", CFG_RIVEN_AUTOGEN_OUTDIR);

    #[cfg(riven_autogen)]
    {
        match std::env::var(RIVEN_AUTOGEN_DEVMODE)
            .ok()
            .map(|s| str::to_ascii_lowercase(&s))
            .as_deref()
        {
            None | Some("outdir") => {
                println!("cargo:rustc-cfg={}", CFG_RIVEN_AUTOGEN_OUTDIR);
                let out_dir = std::env::var_os("OUT_DIR").unwrap();
                autogen::autogen(&*out_dir, &*out_dir);
            }
            Some("src") => {
                autogen::autogen("src", "src/consts");
            }
            Some("none") => {
                // Do nothing.
            }
            Some(other) => {
                panic!("Unknown {}: {:?}", RIVEN_AUTOGEN_DEVMODE, other);
            }
        }
    }
}
