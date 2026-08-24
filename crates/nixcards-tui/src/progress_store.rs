use nixcards_core::ProgressFile;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

pub fn default_path() -> Result<PathBuf, String> {
    if let Some(state_home) = env::var_os("XDG_STATE_HOME") {
        return Ok(PathBuf::from(state_home).join("nixcards/progress.json"));
    }
    let home = env::var_os("HOME").ok_or("HOME and XDG_STATE_HOME are both unset")?;
    Ok(PathBuf::from(home).join(".local/state/nixcards/progress.json"))
}

pub fn load(path: &Path) -> Result<ProgressFile, String> {
    if !path.exists() {
        return Ok(ProgressFile::default());
    }
    let json = fs::read_to_string(path)
        .map_err(|error| format!("cannot read {}: {error}", path.display()))?;
    ProgressFile::from_json(&json).map_err(|error| format!("{}: {error}", path.display()))
}

pub fn save(path: &Path, progress: &ProgressFile) -> Result<(), String> {
    let parent = path
        .parent()
        .ok_or_else(|| format!("{} has no parent directory", path.display()))?;
    fs::create_dir_all(parent)
        .map_err(|error| format!("cannot create {}: {error}", parent.display()))?;
    let temporary = path.with_extension("json.tmp");
    fs::write(&temporary, progress.to_json_pretty()?)
        .map_err(|error| format!("cannot write {}: {error}", temporary.display()))?;
    fs::rename(&temporary, path)
        .map_err(|error| format!("cannot replace {}: {error}", path.display()))
}

pub fn import(source: &Path, destination: &Path) -> Result<ProgressFile, String> {
    let progress = load(source)?;
    save(destination, &progress)?;
    Ok(progress)
}

pub fn export(source: &Path, destination: &Path) -> Result<(), String> {
    let progress = load(source)?;
    let parent = destination
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty());
    if let Some(parent) = parent {
        fs::create_dir_all(parent)
            .map_err(|error| format!("cannot create {}: {error}", parent.display()))?;
    }
    fs::write(destination, progress.to_json_pretty()?)
        .map_err(|error| format!("cannot write {}: {error}", destination.display()))
}
