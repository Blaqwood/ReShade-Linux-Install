use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::error::Error;
use reqwest as request;
use goblin::pe::PE;
use tempdir::TempDir;

static temp_dir: TempDir = todo!();
pub static Repos: Vec<Repo> = vec![
    Repo {url: "", name: ""},
    Repo {url: "", name: ""},
    Repo {url: "", name: ""},
    Repo {url: "", name: ""},
    Repo {url: "", name: ""},
    Repo {url: "", name: ""},
    Repo {url: "", name: ""},
    Repo {url: "", name: ""},
];
pub struct Repo {
    pub url: &'static str,
    pub name: &'static str,
}

pub fn extract_zip_file(file_path: &Path) -> Result<(), Box<dyn Error>> {
    let file = File::open(setup_file_path)?;
    let dest = temp_dir.path().join("dll");

    zip_extract::extract(file, &dest, true)?;
    return Ok(());
}

fn extract_zip_file(repo: &Repo) -> Result<(), Box<dyn Error>> {
    let path = temp_dir.path().join("zip").join(repo.name);
    let file = File::open(&path)?;

    zip_extract::extract(file, temp_dir.join("reshade-shaders"), true)?;
    return Ok(());
}

// download a shader
async fn download_repo(repo: &Repo, client: &request::Client) -> Result<(), Box<dyn Error>> {
    let request = client.get(url).send().await?;
    
    let path = temp_dir.path().join("zip").join(repo.name);
    let mut file = 

    client.execute(request).await?;

    return Ok(());
}

pub fn download_all(repos: &[Repo], destination: &Path) -> Result<(), Box<dyn Error>> {
    let client = request::Client::new();
    
    for repo in repos {
        download_repo(repo, &client)?;
    }

    return Ok(());
}

fn generate_config(repos: &[&Repo], game_exe_path: &Path) -> String {
    let mut conf = ini::Ini::new();

    conf.with_section(Some("GENERAL"))
        .set("EffectSearchPaths", todo!())
        .set("TextureSearchPaths", ".\\Textures")
        .set("PreprocessorDefinitions", todo!());
    conf.with_section(Some("INPUT"))
        .set("KeyOverlay", "36,0,0,0");
    
    return String::from(conf);
}

fn write_config(text: &str, game_exe_path: &Path) -> Result<(), Box<dyn Error>> {
    let data = text.as_bytes();
    let config_path = game_exe_path.join("Reshade.ini");
    let mut file = File::create(&config_path)?;

    return file.write_all(data);
}

fn install_dll(dll_name: &str, game_exe_path: &Path) -> Result<(), Box<dyn Error>> {
    let dll_path = temp_dir.path().join("dll").join(dll_name);
    let dll_game_path = game_exe_path.join(dll_name);

    std::fs::copy(&dll_path, &dll_game_path)?;
    return Ok(());
}