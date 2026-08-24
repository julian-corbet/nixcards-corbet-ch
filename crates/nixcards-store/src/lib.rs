use nixcards_core::{CATALOG_INDEX_SCHEMA_VERSION, Catalog, CatalogIndex};
use std::collections::BTreeSet;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub const OFFICIAL_REPOSITORY: &str = "https://github.com/julian-corbet/nixcards-corbet-ch.git";
pub const CATALOG_BRANCH: &str = "catalog";
pub const CATALOG_INDEX_FILE: &str = "catalog.json";

#[derive(Debug, Clone)]
pub struct CatalogStore {
    root: PathBuf,
}

impl CatalogStore {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn is_initialized(&self) -> bool {
        self.root.join(".git").exists() && self.root.join(CATALOG_INDEX_FILE).is_file()
    }

    pub fn initialize(&self, repository: &str) -> Result<(), String> {
        if self.root.exists() {
            let mut entries = fs::read_dir(&self.root)
                .map_err(|error| format!("cannot read {}: {error}", self.root.display()))?;
            if entries.next().is_some() {
                return Err(format!(
                    "{} already exists and is not an initialized nixcards store",
                    self.root.display()
                ));
            }
        }
        let parent = self
            .root
            .parent()
            .ok_or_else(|| format!("{} has no parent directory", self.root.display()))?;
        fs::create_dir_all(parent)
            .map_err(|error| format!("cannot create {}: {error}", parent.display()))?;
        run(
            Command::new("git")
                .arg("clone")
                .arg("--filter=blob:none")
                .arg("--sparse")
                .arg("--single-branch")
                .arg("--branch")
                .arg(CATALOG_BRANCH)
                .arg(repository)
                .arg(&self.root),
            "initialize nixcards catalogue",
        )?;
        self.validate_checkout()
    }

    pub fn validate_checkout(&self) -> Result<(), String> {
        if !self.root.join(".git").exists() {
            return Err(format!("{} is not a Git checkout", self.root.display()));
        }
        self.catalog_index().map(|_| ())
    }

    pub fn catalog_index(&self) -> Result<CatalogIndex, String> {
        let path = self.root.join(CATALOG_INDEX_FILE);
        let raw = fs::read_to_string(&path)
            .map_err(|error| format!("cannot read {}: {error}", path.display()))?;
        let index: CatalogIndex = serde_json::from_str(&raw)
            .map_err(|error| format!("cannot parse {}: {error}", path.display()))?;
        validate_index(&index)?;
        Ok(index)
    }

    pub fn selected_paths(&self) -> Result<Vec<String>, String> {
        self.validate_checkout()?;
        let output = git_output(&self.root, ["sparse-checkout", "list"])?;
        let mut paths: Vec<_> = output
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(normalize_path)
            .collect();
        paths.sort();
        paths.dedup();
        Ok(paths)
    }

    pub fn selected_set_ids(&self) -> Result<Vec<String>, String> {
        let index = self.catalog_index()?;
        let paths: BTreeSet<_> = self.selected_paths()?.into_iter().collect();
        Ok(index
            .sets
            .into_iter()
            .filter(|set| paths.contains(&set.path))
            .map(|set| set.id)
            .collect())
    }

    pub fn resolve_selectors(&self, selectors: &[String]) -> Result<Vec<String>, String> {
        resolve_selectors(&self.catalog_index()?, selectors)
    }

    pub fn select(&self, set_ids: &[String]) -> Result<(), String> {
        let index = self.catalog_index()?;
        let known: std::collections::BTreeMap<_, _> = index
            .sets
            .into_iter()
            .map(|set| (set.id, set.path))
            .collect();
        let mut paths = BTreeSet::new();
        for id in set_ids {
            let path = known
                .get(id)
                .ok_or_else(|| format!("unknown card set {id}"))?;
            paths.insert(path.clone());
        }
        let input = paths.into_iter().collect::<Vec<_>>().join("\n");
        git_input(
            &self.root,
            ["sparse-checkout", "set", "--cone", "--stdin"],
            &input,
        )?;
        Ok(())
    }

    pub fn sync(&self) -> Result<(), String> {
        self.validate_checkout()?;
        let dirty = git_output(
            &self.root,
            ["status", "--porcelain", "--untracked-files=no"],
        )?;
        if !dirty.trim().is_empty() {
            return Err(
                "catalogue checkout has local changes; edit the source repository instead".into(),
            );
        }
        git(
            &self.root,
            ["fetch", "--filter=blob:none", "origin", CATALOG_BRANCH],
        )?;
        git(&self.root, ["merge", "--ff-only", "FETCH_HEAD"])?;
        self.validate_checkout()
    }

    pub fn load_selected_catalog(&self) -> Result<Catalog, String> {
        let index = self.catalog_index()?;
        let selected: BTreeSet<_> = self.selected_paths()?.into_iter().collect();
        let mut sources = Vec::new();
        for set in index.sets {
            if selected.contains(&set.path) {
                collect_markdown(&self.root, &self.root.join(&set.path), &mut sources)?;
            }
        }
        Catalog::from_sources(
            sources
                .iter()
                .map(|(path, source)| (path.as_str(), source.as_str())),
        )
        .map_err(|error| error.to_string())
    }

    pub fn promisor_filter(&self) -> Result<Option<String>, String> {
        let value = git_output_optional(
            &self.root,
            ["config", "--get", "remote.origin.partialclonefilter"],
        )?;
        Ok(value.map(|value| value.trim().to_owned()))
    }
}

pub fn default_store_path() -> Result<PathBuf, String> {
    if let Some(path) = env::var_os("NIXCARDS_STORE").filter(|path| !path.is_empty()) {
        return Ok(PathBuf::from(path));
    }
    let base = if let Some(path) = env::var_os("XDG_DATA_HOME").filter(|path| !path.is_empty()) {
        PathBuf::from(path)
    } else {
        let home = env::var_os("HOME").ok_or("HOME and XDG_DATA_HOME are both unset")?;
        PathBuf::from(home).join(".local/share")
    };
    Ok(base.join("nixcards/knowledge/cards"))
}

pub fn resolve_selectors(
    index: &CatalogIndex,
    selectors: &[String],
) -> Result<Vec<String>, String> {
    let mut selected = BTreeSet::new();
    for raw in selectors {
        let selector = raw.trim().trim_end_matches(".*");
        if selector.is_empty() {
            return Err("an empty catalogue selector is not valid".into());
        }
        let matches: Vec<_> = index
            .sets
            .iter()
            .filter(|set| {
                set.id == selector
                    || set
                        .id
                        .strip_prefix(selector)
                        .is_some_and(|suffix| suffix.starts_with('.'))
            })
            .map(|set| set.id.clone())
            .collect();
        if matches.is_empty() {
            return Err(format!("catalogue selector {raw:?} matches no card sets"));
        }
        selected.extend(matches);
    }
    Ok(selected.into_iter().collect())
}

pub fn validate_index(index: &CatalogIndex) -> Result<(), String> {
    if index.schema_version != CATALOG_INDEX_SCHEMA_VERSION {
        return Err(format!(
            "unsupported catalogue index schema {}; expected {}",
            index.schema_version, CATALOG_INDEX_SCHEMA_VERSION
        ));
    }
    let mut ids = BTreeSet::new();
    let mut paths = BTreeSet::new();
    for set in &index.sets {
        let expected = set.id.replace('.', "/");
        if set.path != expected {
            return Err(format!(
                "card set {} uses path {}; expected {}",
                set.id, set.path, expected
            ));
        }
        if !ids.insert(&set.id) {
            return Err(format!("duplicate card set ID {}", set.id));
        }
        if !paths.insert(&set.path) {
            return Err(format!("duplicate card set path {}", set.path));
        }
    }
    Ok(())
}

fn collect_markdown(
    root: &Path,
    directory: &Path,
    sources: &mut Vec<(String, String)>,
) -> Result<(), String> {
    let entries = fs::read_dir(directory)
        .map_err(|error| format!("cannot read {}: {error}", directory.display()))?;
    for entry in entries {
        let path = entry
            .map_err(|error| format!("cannot read {}: {error}", directory.display()))?
            .path();
        if path.is_dir() {
            collect_markdown(root, &path, sources)?;
        } else if path.extension() == Some(OsStr::new("md")) {
            let relative = path
                .strip_prefix(root)
                .map_err(|error| format!("cannot relativize {}: {error}", path.display()))?;
            let source = fs::read_to_string(&path)
                .map_err(|error| format!("cannot read {}: {error}", path.display()))?;
            sources.push((
                format!("cards/{}", normalize_path(&relative.to_string_lossy())),
                source,
            ));
        }
    }
    sources.sort_by(|left, right| left.0.cmp(&right.0));
    Ok(())
}

fn normalize_path(path: &str) -> String {
    path.trim().replace('\\', "/").trim_matches('/').to_owned()
}

fn git<const N: usize>(root: &Path, args: [&str; N]) -> Result<(), String> {
    run(
        Command::new("git").arg("-C").arg(root).args(args),
        "run Git",
    )
}

fn git_output<const N: usize>(root: &Path, args: [&str; N]) -> Result<String, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .map_err(|error| format!("cannot run git: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "git failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    String::from_utf8(output.stdout).map_err(|error| format!("git returned invalid UTF-8: {error}"))
}

fn git_output_optional<const N: usize>(
    root: &Path,
    args: [&str; N],
) -> Result<Option<String>, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .map_err(|error| format!("cannot run git: {error}"))?;
    if output.status.success() {
        return String::from_utf8(output.stdout)
            .map(Some)
            .map_err(|error| format!("git returned invalid UTF-8: {error}"));
    }
    if output.status.code() == Some(1) {
        return Ok(None);
    }
    Err(format!(
        "git failed: {}",
        String::from_utf8_lossy(&output.stderr).trim()
    ))
}

fn git_input<const N: usize>(root: &Path, args: [&str; N], input: &str) -> Result<(), String> {
    let mut child = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| format!("cannot run git: {error}"))?;
    child
        .stdin
        .take()
        .ok_or("cannot open git stdin")?
        .write_all(input.as_bytes())
        .map_err(|error| format!("cannot write git input: {error}"))?;
    let output = child
        .wait_with_output()
        .map_err(|error| format!("cannot wait for git: {error}"))?;
    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "git failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ))
    }
}

fn run(command: &mut Command, action: &str) -> Result<(), String> {
    let output = command
        .output()
        .map_err(|error| format!("cannot {action}: {error}"))?;
    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "cannot {action}: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nixcards_core::CatalogIndexSet;
    use tempfile::TempDir;

    fn index() -> CatalogIndex {
        CatalogIndex {
            schema_version: CATALOG_INDEX_SCHEMA_VERSION,
            sets: vec![
                CatalogIndexSet {
                    id: "cloud.bearingpoint.interview".into(),
                    title: "Interview".into(),
                    language: "en".into(),
                    tags: vec!["cloud".into()],
                    path: "cloud/bearingpoint/interview".into(),
                    card_count: 1,
                },
                CatalogIndexSet {
                    id: "cloud.certificates.databricks.introduction".into(),
                    title: "Databricks".into(),
                    language: "en".into(),
                    tags: vec!["certificate".into()],
                    path: "cloud/certificates/databricks/introduction".into(),
                    card_count: 1,
                },
            ],
        }
    }

    #[test]
    fn category_selector_resolves_only_its_descendant_sets() {
        let selected = resolve_selectors(&index(), &["cloud.certificates.databricks".into()])
            .expect("selector should resolve");
        assert_eq!(selected, ["cloud.certificates.databricks.introduction"]);
    }

    #[test]
    fn index_paths_must_mirror_dotted_ids() {
        let mut broken = index();
        broken.sets[0].path = "wrong/path".into();
        assert!(validate_index(&broken).unwrap_err().contains("expected"));
    }

    #[test]
    fn blobless_sparse_checkout_materializes_only_selected_sets() {
        let fixture = TempDir::new().expect("fixture");
        let remote = fixture.path().join("catalog.git");
        let source = fixture.path().join("source");
        git_ok(Command::new("git").arg("init").arg("--bare").arg(&remote));
        git_ok(Command::new("git").arg("-C").arg(&remote).args([
            "config",
            "uploadpack.allowFilter",
            "true",
        ]));
        git_ok(
            Command::new("git")
                .arg("init")
                .arg("--initial-branch=catalog")
                .arg(&source),
        );
        git_ok(Command::new("git").arg("-C").arg(&source).args([
            "config",
            "user.email",
            "fixture@example.invalid",
        ]));
        git_ok(
            Command::new("git")
                .arg("-C")
                .arg(&source)
                .args(["config", "user.name", "Fixture"]),
        );
        fs::write(
            source.join(CATALOG_INDEX_FILE),
            format!(
                "{}\n",
                serde_json::to_string_pretty(&index()).expect("serialize index")
            ),
        )
        .expect("write index");
        for set in &index().sets {
            let directory = source.join(&set.path);
            fs::create_dir_all(&directory).expect("create set");
            fs::write(
                directory.join("set.md"),
                format!(
                    "---\nid: {}\ntitle: {}\nlanguage: en\nlicense: CC-BY-NC-SA-4.0\nattribution: Fixture\ntags: [test]\nsources: [https://example.invalid/source]\n---\n",
                    set.id, set.title
                ),
            )
            .expect("write set");
            fs::write(directory.join("question.md"), "# Question?\n\nAnswer.\n")
                .expect("write card");
        }
        git_ok(
            Command::new("git")
                .arg("-C")
                .arg(&source)
                .args(["add", "."]),
        );
        git_ok(
            Command::new("git")
                .arg("-C")
                .arg(&source)
                .args(["commit", "-m", "fixture"]),
        );
        git_ok(
            Command::new("git")
                .arg("-C")
                .arg(&source)
                .arg("remote")
                .arg("add")
                .arg("origin")
                .arg(&remote),
        );
        git_ok(
            Command::new("git")
                .arg("-C")
                .arg(&source)
                .args(["push", "origin", "catalog"]),
        );

        let checkout = fixture.path().join("knowledge/cards");
        let store = CatalogStore::new(&checkout);
        store
            .initialize(&file_url(&remote))
            .expect("initialize sparse store");
        assert!(!checkout.join("cloud/bearingpoint").exists());
        assert!(!checkout.join("cloud/certificates").exists());

        store
            .select(&["cloud.certificates.databricks.introduction".into()])
            .expect("select databricks");
        assert!(
            checkout
                .join("cloud/certificates/databricks/introduction/question.md")
                .is_file()
        );
        assert!(!checkout.join("cloud/bearingpoint").exists());
        assert_eq!(
            store.promisor_filter().unwrap().as_deref(),
            Some("blob:none")
        );
        assert_eq!(store.load_selected_catalog().unwrap().sets.len(), 1);
    }

    fn git_ok(command: &mut Command) {
        let output = command.output().expect("run git fixture command");
        assert!(
            output.status.success(),
            "git fixture command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    fn file_url(path: &Path) -> String {
        let normalized = path.to_string_lossy().replace('\\', "/");
        if cfg!(windows) {
            format!("file:///{normalized}")
        } else {
            format!("file://{normalized}")
        }
    }
}
