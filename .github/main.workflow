workflow "New workflow" {
  resolves = ["Rust Action 2", "GitHub Action for AWS"]
  on = "push"
}

action "Rust Action" {
  uses = "icepuma/rust-action@1.0.4"
  runs = "cargo"
  args = "build"
}

action "Rust Action 2" {
  uses = "icepuma/rust-action@1.0.4"
  runs = "cargo"
  args = "build"
}

action "GitHub Action for AWS" {
  uses = "actions/aws/cli@efb074ae4510f2d12c7801e4461b65bf5e8317e6"
  needs = ["Rust Action", "Rust Action 2"]
}
