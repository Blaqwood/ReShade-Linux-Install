use std::env;
use std::thread;
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

enum Api {
    Dxgi,
    D3d9,
    Opengl
}

pub struct Install<'a> {
    game_path: PathBuf,
    shader_names: Vec<&'a str>,
    shader_urls: Vec<&'a str>,
    api: Api,
}

impl Install {
    fn download_default(&self) -> Result<(), &'static str> {
        std::process::Command::new("git").args([
            "-C",
            self.game_path,
            "clone",
            self.shader_urls[0]
        ]).spawn()?;
        Ok(())
    }

    fn download(&self, url: &str) -> io::Result<()> {
        std::process::Command::new("git").args(["-C", "clone", url]).spawn()?;
        Ok(())
    }

    // download the shader packs from the users input
    fn download_all(&self) -> Result<(), &'static str> {
        let dl_path = self.game_path.join("reshade-shaders").join("");
        let mut clone = Command::new("git");

        for url in self.shader_urls.iter().skip(1) {
            // download each repo
            self.download(url)?;
        }
        
        setup_shaders()?;
        Ok(())
    }

    fn install_exists(&self) -> bool {
        self.game_path.join("dxgi.dll").exists() || self.game_path.join("d3d9.dll").exists() || self.game_path.join("opengl32.dll").exists() || self.game_path.join("opengl64.dll").exists()
    }
    
    fn create_config_file(shaders: Vec<&str>) -> Result<(), &'static str> {    
        const DEFAULT: &str = r"[GENERAL]
PreprocessorDefinitions=RESHADE_DEPTH_LINEARIZATION_FAR_PLANE=1000.0,ESHADE_DEPTH_INPUT_IS_UPSIDE_DOWN=0,RESHADE_DEPTH_INPUT_IS_REVERSED=0,RESHADE_DEPTH_INPUT_IS_LOGARITHMIC=0
EffectSearchPaths=.\reshade-shaders\Shaders
TextureSearchPaths=.\reshade-shaders\Textures

[INPUT]
KeyOverlay=36,0,0,0";

        let mut file =  match File::create(Path::new(RESHADE).join("ReShade.ini")) {
            Ok(f) => f,
            Err(..) => return Err("Failed to create config file"),
        };

        
        let shader_names = shaders.join(",");

        let effects_str = DEFAULT.replace(
            r"EffectSearchPaths=.\reshade-shaders\Shaders",
            format!("EffectSearchPaths={}", shader_names).as_str()
        );

        match file.write_all(effects_str.as_bytes()) {
            Ok(..) => Ok(()),
            Err(..) => Err("Failed to write to file"),    
        }
        Ok(())
    }
}

fn parse_shaders() -> {

}

fn setup() -> Result<(), std::io::Error> {
    std::fs::create_dir_all(Path::new(TMP).join("shaders"))
}

// extract the dll files from the setup executable
pub fn extract_dll(path: &Path) -> Result<(), &'static str> {
    if let Ok(zip) = File::open(path) {
        return match zip_extract::extract(zip, Path::new(TMP), true) {
            Ok(..) => Ok(()),
            Err(e) => Err("Failed to extract DLL")
        };
    }
    else {
        return Err("Failed to open file");
    }
}

pub fn download_default() -> Result<(), &'static str> {
    const SHADER_URL: &str = "https://github.com/crosire/reshade-shaders/";
    // download this ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ then
    match std::process::Command::new("git").args(["-C", RESHADE, "clone", SHADER_URL]).status() {
        Ok(..) => Ok(()),
        Err(e) => {
            use std::io::ErrorKind;
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

pub fn configure(game_path: &Path, api: &str) -> Result<(), &'static str> {
    fn is_x86_64(exe_data: &[u8]) -> Result<bool, Box<dyn Error>> {
        use goblin::Object;
    
        match Object::parse(exe_data)? {
            Object::PE(pe) => Ok(pe.is_64),
            _ => Err("File is not a Windows PE file.".into()),
        }
    }

    let exe = match read_file(game_path) {
        Ok(bytes) => bytes,
        Err(..) => return Err("Failed read exe")
    };
    
    match is_x86_64(&exe) {
        Ok(b) => todo!(),
        _ => todo!()
    }

    Ok(())

}

fn setup_shaders() -> Result<(), &'static str> {    
    
    fn setup_folder(folder: walkdir::DirEntry) -> Result<(), &'static str> {
        let folder_name = Path::new("/tmp/reshade/reshade-shaders/Shaders").join(folder.file_name());
        let options = fs_extra::dir::CopyOptions::new();

        if create_dir(folder_name.as_path(), false).is_err() {
            return Err("Failed to create directory");
        }

        // Shaders
        if copy_dir(
            folder.path().join("Shaders"),
            folder_name.as_path(),
            &options
        ).is_err() {
            return Err("Failed to setup shader");
        }
        
        // Textures
        if copy_dir(
            folder.path().join("Textures"),
            "/tmp/reshade/reshade-shaders/Textues",
            &options
        ).is_err() {
            return Err("Failed to setup shader");
        }
        Ok(())
    }

    for item in WalkDir::new("/tmp/reshade/tmp/shaders") {
        if let Ok(folder) = item {
            setup_folder(folder)?;
        }
        else {
            return Err("Failed to read directory");
        }
    }
    
    //cleanup
    
    Ok(())
}

pub fn install(game_path: &Path) -> Result<&'static str, &'static str> {
    match copy_dir("/tmp/reshade", game_path, &fs_extra::dir::CopyOptions::new()) {
        Ok(..) => Ok("ReShade Installed successfully"),
        Err(..) => Err("Failed to install ReShade"),
    }
}



fn create_config_file(shaders: Vec<&str>) -> Result<(), &'static str> {    
    const DEFAULT: &str = r"[GENERAL]
PreprocessorDefinitions=RESHADE_DEPTH_LINEARIZATION_FAR_PLANE=1000.0,ESHADE_DEPTH_INPUT_IS_UPSIDE_DOWN=0,RESHADE_DEPTH_INPUT_IS_REVERSED=0,RESHADE_DEPTH_INPUT_IS_LOGARITHMIC=0
EffectSearchPaths=.\reshade-shaders\Shaders
TextureSearchPaths=.\reshade-shaders\Textures

[INPUT]
KeyOverlay=36,0,0,0";

    let mut file =  match File::create(Path::new(RESHADE).join("ReShade.ini")) {
        Ok(f) => f,
        Err(..) => return Err("Failed to create config file"),
    };

    
    let shader_names = shaders.join(",");

    let effects_str = DEFAULT.replace(
        r"EffectSearchPaths=.\reshade-shaders\Shaders",
        format!("EffectSearchPaths={}", shader_names).as_str()
    );

    match file.write_all(effects_str.as_bytes()) {
        Ok(..) => Ok(()),
        Err(..) => Err("Failed to write to file"),    
    }
    }
    Ok(())
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

pub fn install() -> Result<(), ()> {
    let cfg = ini::Ini::load_from_file("/tmp/reshade.ini")?;

    let x = cfg[2];
}
    