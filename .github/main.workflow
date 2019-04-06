workflow "New workflow" {
  resolves = ["Build", "Build all_features", "Tests"]
  on = "push"
}

action "Build" {
  uses = "docker://rust:latest"
  runs = "cargo"
  args = "build"
}

action "Build all_features" {
  uses = "docker://rust:latest"
  runs = "cargo"
  args = "build --all-features"
}
 
action "Tests" {
  uses = "docker://rust:latest"
  runs = "cargo"
  args = "test --all-features"
  needs = ["Build", "Build all_features"]
}
