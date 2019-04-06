workflow "New workflow" {
  resolves = ["Build"]
  on = "push"
}

action "Build" {
  uses = "rust/latest"
  runs = "cargo"
  args = "build"
}

action "Build all_features" {
  uses = "rust/latest"
  runs = "cargo"
  args = "build --all-features"
}

action "Tests" {
  uses = "rust/latest"
  needs = ["Cargo build", "Cargo build all_features"]
}
