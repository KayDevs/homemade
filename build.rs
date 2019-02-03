use std::env;
use std::path::{Path, PathBuf};
use std::io::prelude::*;
use std::fs::File;

//function for capitalizing strings
pub trait Capitalize {
  fn capitalize(&self) -> String;
}

impl Capitalize for String {
  fn capitalize(&self) -> String {
    let mut c = self.chars();
    match c.next() {
      None => String::new(),
      Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
  }
}

fn main() {
    let target = env::var("TARGET").unwrap();
    if target.contains("pc-windows") {
        let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let mut lib_dir = manifest_dir.clone();
        let mut dll_dir = manifest_dir.clone();
        if target.contains("msvc") {
            lib_dir.push("msvc");
            dll_dir.push("msvc");
        }
        else {
            lib_dir.push("gnu-mingw");
            dll_dir.push("gnu-mingw");
        }
        lib_dir.push("lib");
        dll_dir.push("dll");
        if target.contains("x86_64") {
            lib_dir.push("64");
            dll_dir.push("64");
        }
        else {
            lib_dir.push("32");
            dll_dir.push("32");
        }
        println!("cargo:rustc-link-search=all={}", lib_dir.display());
        for entry in std::fs::read_dir(dll_dir).expect("Can't read DLL dir")  {
            let entry_path = entry.expect("Invalid fs entry").path();
            let file_name_result = entry_path.file_name();
            let mut new_file_path = manifest_dir.clone();
            if let Some(file_name) = file_name_result {
                let file_name = file_name.to_str().unwrap();
                if file_name.ends_with(".dll") {
                    new_file_path.push(file_name);
                    std::fs::copy(&entry_path, new_file_path.as_path()).expect("Can't copy from DLL dir");
                }
            }
        }
    } else {
        println!("cargo:rustc-link-lib=sndio"); 
    }

    //'resources' folder codegen
    //
    let out_dir = env::var("OUT_DIR").unwrap();
    let path = Path::new(&out_dir).join("resources.rs");
    let mut resources_out = File::create(&path).expect("cannot read resources.rs");
    let resources_dir = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("resources/");
    struct Entry {
        filename: String,
        bigname: String,
    }
    let mut bmps: Vec<Entry> = Vec::new();
    
    //collect resources
    for resource in std::fs::read_dir(resources_dir).expect("cannot read resources dir") {
        let resource = resource.unwrap();
        let bigname = resource.path().file_stem().unwrap().to_str().unwrap().to_ascii_uppercase();
        let file_name = resource.file_name();
        let fname = file_name.to_str().unwrap();
        if fname.ends_with(".bmp") {
            bmps.push(Entry{filename: fname.to_string(), bigname: bigname});
        }
    }

    //create source file
    resources_out.write_all(b"
mod resources {
    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    pub enum Sprites {
    ").unwrap();
        for e in &bmps {
            resources_out.write_fmt(format_args!("\t{},", e.bigname.clone().to_ascii_lowercase().capitalize())).unwrap();
        }    
        resources_out.write_all(b"
    }
    ").unwrap();

    resources_out.write_all(b"
    use sdl2::render::Texture;
    pub struct Resources {
        sprites: Vec<Texture>,
    }
    ").unwrap();

    resources_out.write_all(b"
    use sdl2::surface::Surface;
    use sdl2::pixels::Color;
    use sdl2::rwops::RWops;
    use sdl2::video::Window;
    use sdl2::render::Canvas;
    impl Resources {
        pub fn new(canvas: &Canvas<Window>) -> Result<Resources, Box<std::error::Error>> {
            let mut r = Resources{sprites: Vec::new()};").unwrap();
    for e in &bmps {
        resources_out.write_fmt(format_args!("
            let mut rwops = RWops::from_bytes(include_bytes!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/resources/{}\")))?;
            let mut surface = Surface::load_bmp_rw(&mut rwops)?;
            surface.set_color_key(true, Color::RGB(255, 0, 255))?;
            let tc = canvas.texture_creator();
            let tex = tc.create_texture_from_surface(&surface)?;
            r.sprites.push(tex);", 
        e.filename.as_str())).unwrap();
    }
    resources_out.write_all(b"
            Ok(r)
        }
    }
    ").unwrap();

    resources_out.write_all(b"
    impl std::ops::Index<Sprites> for Resources {
        type Output = Texture;
        fn index(&self, s: Sprites) -> &Texture {
            match s {").unwrap();
    for (i, e) in bmps.iter().enumerate() {
        resources_out.write_fmt(format_args!("
                Sprites::{} => &self.sprites[{}],", e.bigname.clone().to_ascii_lowercase().capitalize(), i)).unwrap();
    }
    resources_out.write_all(b"
            }
        }
    }").unwrap();

    resources_out.write_all(b"
    impl std::ops::IndexMut<Sprites> for Resources {
        fn index_mut(&mut self, s: Sprites) -> &mut Texture {
            match s {").unwrap();
    for (i, e) in bmps.iter().enumerate() {
        resources_out.write_fmt(format_args!("
                Sprites::{} => &mut self.sprites[{}],", e.bigname.clone().to_ascii_lowercase().capitalize(), i)).unwrap();
    }
    resources_out.write_all(b"
            }
        }
    }").unwrap();

    resources_out.write_all(b"
}").unwrap();
}