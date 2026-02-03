// Rename template files.
file::rename(".github/workflows/release.yaml.template", ".github/workflows/release.yaml");
file::rename("Cargo.toml.template", "Cargo.toml");
file::rename("README.md.template", "README.md");

// initialize git repository
system::command("git", ["init"]);
