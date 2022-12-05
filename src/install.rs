/* use std::io;
use std::error::Error;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::fs::{File, read as read_file};
use std::process::Command;
use std::io::ErrorKind;

use fs_extra::{
    file::{copy as copy_file, write_all as write_file},
    dir::{copy as copy_dir, create_all as create_dir}
};
use walkdir::WalkDir;

const RESHADE: &str = "/tmp/reshade";
const TMP: &str = "/tmp/reshade/tmp";
*/
pub const SHADERS: [(&str, &str); 3] = [
    ("Standard Effects", "https://github.com/"),
    ("SweetFX by Ceejay.dk", "https://github.com/CeeJayDK/SweetFX"),
    ("AstrayFX by BlueSkyDefender", "https://blueskydefender.github.io/AstrayFX/"),
/*  ("", ""),
    ("", ""),
    ("", ""),
    ("", ""),
    ("", ""),
    ("", "") */
];
/*
#[allow(non_camel_case_types)]
enum Api {
    dxgi,
    d3d9,
    opengl
}

#[allow(non_camel_case_types)]
enum Bit {
    x32,
    x64,
}

pub struct Install {
    game_path: PathBuf,
    shader_urls: Vec<String>,
    api: Api,
}

impl Install {
    // download the shader packs from the users input
    fn download(&self) -> Result<(), &'static str> {
        let shader_path = self.game_path.join("reshade-shaders/tmp");
        let mut clone = Command::new("git");

        // download the first one as a base
        clone.args(["-C", self.game_path.to_str().unwrap(), "clone", &self.shader_urls[0]]).spawn()?;
        
        // download crosire reposiory
        std::fs::create_dir("/tmp/reshade-shaders/etc")?;

        for url in self.shader_urls.iter().skip(1) {
            clone.args(["-C", "/tmp/reshade-shaders/etc", "clone", url]).spawn()?;
        }
        
        setup_shaders()?;
        Ok(())
    }

    

    pub fn install_exists(&self) -> bool {
        self.game_path.join("dxgi.dll").exists() || self.game_path.join("d3d9.dll").exists() || self.game_path.join("opengl32.dll").exists() || self.game_path.join("opengl64.dll").exists()
    }
    
    fn create_config_file(&self, shaders: Vec<&str>) -> Result<(), &'static str> {    
        const DEFAULT: &str = r"[GENERAL]
PreprocessorDefinitions=RESHADE_DEPTH_LINEARIZATION_FAR_PLANE=1000.0,ESHADE_DEPTH_INPUT_IS_UPSIDE_DOWN=0,RESHADE_DEPTH_INPUT_IS_REVERSED=0,RESHADE_DEPTH_INPUT_IS_LOGARITHMIC=0
EffectSearchPaths=.\reshade-shaders\Shaders
TextureSearchPaths=.\reshade-shaders\Textures

[INPUT]
KeyOverlay=36,0,0,0";

        let mut file =  File::create(Path::new(RESHADE).join("ReShade.ini"))?;
        
        let shader_names = self.shader_names.join("");

        file.write_all(effects_str.as_bytes())?;
        Ok(())
    }

    fn get_bit(&self) -> Result<Bit, &'static str> {
        use goblin::Object;
    
        if let Ok(Object::PE(exe)) = Object::parse(read_file(self.game_path)?.as_ref()) {
            match exe.is_64 {
                true => Ok(Bit::x64),
                false => Ok(Bit::x32),
            }
        }
        else {
            Err("Faeiled to read file")
        }
   }
}

fn setup() -> Result<(), std::io::Error> {
    std::fs::create_dir_all(Path::new(TMP).join("shaders"))
}

// extract the dll files from the setup executable
pub fn extract_dll(path: &Path) -> Result<(), &'static str> {
    let zip = File::open(path)?;
    zip_extract::extract(zip, Path::new(TMP), true)?;
    Ok(())
}

pub fn download_default() -> Result<(), &'static str> {
    const SHADER_URL: &str = "https://github.com/crosire/reshade-shaders/";
    // download this ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ then
    match std::process::Command::new("git").args(["-C", RESHADE, "clone", SHADER_URL]).status() {
        Ok(..) => Ok(()),
        Err(e) => {
            match e.kind() {
                ErrorKind::NotFound => Err("git is not installed"),
                _ => Err("Failed to run command"),
            }
        }
    }
}

// download the shader packs from the users input
pub fn download(repos: Vec<String>) -> Result<(), &'static str> {
    const DEST: &str = "/tmp/reshade/tmp/shaders";
    
    download_default()?;

    let mut clone = Command::new("git");
    
    for url in repos {
        // download each repo
        if clone.args(["-C", DEST, "clone", url.as_str()]).status().is_err() {
            return Err("One or more shaders failed to download");
        }
    }
    setup_shaders()?;
    Ok(())
}



pub fn install(game_path: &Path) -> Result<&'static str, &'static str> {
    match copy_dir("/tmp/reshade", game_path, &fs_extra::dir::CopyOptions::new()) {
        Ok(..) => Ok("ReShade Installed successfully"),
        Err(..) => Err("Failed to install ReShade"),
    }
}

pub fn cleaup() -> io::Result<()> {
    std::fs::remove_dir_all("/tmp/reshade")?;
    Ok(())
}

const urls1 : &str = "https://github.com/crosire/reshade-shaders";
const urls2 : &str = "https://github.com/CeeJayDK/SweetFX";

fn is_git_installed() -> bool {
    which::which("git").is_ok()
}
     */