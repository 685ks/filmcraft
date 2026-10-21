//! Optional craft-fonts build input (see 685ks/craft-fonts `docs/integration.md` and craftrules
//! `standards/fonts.md`), through the shared `craft-fonts-build` helper.
//!
//! With `CRAFT_FONTS_DIR=<absolute path of a craft-fonts checkout>` this embeds every font listed
//! in its `fonts/manifest.txt` as [`CRAFT_FONTS`](crate::fonts::CRAFT_FONTS); unset, `CRAFT_FONTS`
//! is empty and nothing else changes. A `CRAFT_FONTS_DIR` that is not a checkout is a warning, or
//! an error with `CRAFT_FONTS_REQUIRED=1` (release builds).
//!
//! Native builds embed every font. The web (wasm32) build embeds only the UI font (BIZ UDPGothic
//! Regular, ~4.5 MB) to stay under the web size budget (Cloudflare serves files up to 25 MiB).
//! Only the local directory is read: no network.

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    // `legacy`: the `CraftFont { family, style, scripts, bytes }` entries this crate's
    // `fonts::CraftFont` defines.
    craft_fonts_build::Builder::new().legacy(true).web_fonts(&[("BIZ UDPGothic", "Regular")]).emit();
}
