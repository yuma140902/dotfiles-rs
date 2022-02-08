use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(name = "dotfiles-rs", author, version, about = "dotfiles organizer")]
struct AppArg {
    #[clap(subcommand)]
    subcommand: AppSubCommand,
}

#[derive(Subcommand, Debug)]
enum AppSubCommand {
    #[clap(visible_alias = "p")]
    /// Pick files or directories to manage dotfile repository
    Pick {
        #[clap(
            long = "repo",
            short,
            default_value = "./repos/dotfiles",
            env = "DOTFILES_REPO"
        )]
        /// dotfilesリポジトリの場所。相対パスの場合はホームディレクトリを基準とする。
        repository: PathBuf,

        #[clap(long = "base", short = 'b', env = "DOTFILES_BASE")]
        /// dotfilesをインストールするディレクトリ。デフォルト値はホームディレクトリ。
        install_base: Option<PathBuf>,

        files_and_dirs: Vec<PathBuf>,
    },
    #[clap(visible_alias = "i")]
    /// Install dotfiles to system
    Install {
        #[clap(
            long = "repo",
            short,
            default_value = "./repos/dotfiles",
            env = "DOTFILES_REPO"
        )]
        /// dotfilesリポジトリの場所。相対パスの場合はホームディレクトリを基準とする。
        repository: PathBuf,

        #[clap(long = "base", short = 'b', env = "DOTFILES_BASE")]
        /// dotfilesをインストールするディレクトリ。デフォルト値はホームディレクトリ。
        install_base: Option<PathBuf>,
    },
}
fn main() {
    let arg = AppArg::parse();
    dbg!(&arg);
    match arg.subcommand {
        AppSubCommand::Pick {
            repository,
            install_base,
            files_and_dirs,
        } => dotfiles::pick(&repository, &install_base, &files_and_dirs),
        AppSubCommand::Install {
            repository,
            install_base,
        } => todo!(),
    }
}
