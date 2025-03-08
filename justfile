set windows-shell := ["pwsh.exe", "-c"]

build_dir := "build"

build:
  cargo build

[windows]
clean:
    if (Test-Path -Path {{build_dir}}) { Remove-Item -LiteralPath {{build_dir}} -Force -Recurse }

[unix]
clean:
    rm -rf {{build_dir}}

fmt:
  cargo fmt
  stylua src

check:
  cargo check

examples:
  -cargo run -q -- examples/*.plua -o {{build_dir}}

