# dotfiles-rs

## 使用方法
```
> dotfiles --help
dotfiles-rs 0.1.0
yuma140902
dotfiles organizer

USAGE:
    dotfiles.exe <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    help       Print this message or the help of the given subcommand(s)
    install    Install dotfiles to system [aliases: i]
    pick       Move files or directories to dotfiles repository [aliases: p]
```
```
> dotfiles help pick
dotfiles.exe-pick
Move files or directories to dotfiles repository

USAGE:
    dotfiles.exe pick [OPTIONS] [FILES_AND_DIRS]...

ARGS:
    <FILES_AND_DIRS>...

OPTIONS:
    -b, --base <INSTALL_BASE>    dotfilesをインストールするディレクトリ。デフォルト値はホームディレクトリ。
                                 [env: DOTFILES_BASE=]
    -h, --help                   Print help information
    -r, --repo <REPOSITORY>      dotfilesリポジトリの場所。相対パスの場合はホームディレクトリを基準とする。
                                 [env: DOTFILES_REPO=] [default: ./repos/dotfiles]
```
```
> dotfiles help install
dotfiles.exe-install
Install dotfiles to system

USAGE:
    dotfiles.exe install [OPTIONS]

OPTIONS:
    -b, --base <INSTALL_BASE>    dotfilesをインストールするディレクトリ。デフォルト値はホームディレクトリ。
                                 [env: DOTFILES_BASE=]
    -h, --help                   Print help information
    -r, --repo <REPOSITORY>      dotfilesリポジトリの場所。相対パスの場合はホームディレクトリを基準とする。
                                 [env: DOTFILES_REPO=] [default: ./repos/dotfiles]
```

## インストール方法
```bash
cargo install --git https://github.com/yuma140902/dotfiles-rs
```
または
```bash
git clone https://github.com/yuma140902/dotfiles-rs.git
cargo install --path ./dotfiles-rs
# rm -rf ./dotfiles-rs
```
