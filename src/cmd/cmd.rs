#![allow(clippy::module_inception)]

use std::path::PathBuf;

use clap::builder::{IntoResettable, Resettable, StyledStr};
use clap::{Parser, Subcommand, ValueEnum, ValueHint};

struct HelpTemplate;

impl IntoResettable<StyledStr> for HelpTemplate {
    fn into_resettable(self) -> Resettable<StyledStr> {
        color_print::cstr!("\
{before-help}<bold><underline>{name} {version}</underline></bold>
{author}
https://github.com/dimarogiv/querypath

{about}

{usage-heading}
{tab}{usage}

{all-args}{after-help}

<bold><underline>Environment variables:</underline></bold>
{tab}<bold>_QP_DATA_DIR</bold>        {tab}Path for querypath data files
{tab}<bold>_QP_ECHO</bold>            {tab}Print the matched directory before navigating to it when set to 1
{tab}<bold>_QP_EXCLUDE_DIRS</bold>    {tab}List of directory globs to be excluded
{tab}<bold>_QP_FZF_OPTS</bold>        {tab}Custom flags to pass to fzf
{tab}<bold>_QP_MAXAGE</bold>          {tab}Maximum total age after which entries start getting deleted
{tab}<bold>_QP_RESOLVE_SYMLINKS</bold>{tab}Resolve symlinks when storing paths").into_resettable()
    }
}

#[derive(Debug, Parser)]
#[clap(
    about,
    author,
    help_template = HelpTemplate,
    disable_help_subcommand = true,
    propagate_version = true,
    version,
)]
pub enum Cmd {
    Add(Add),
    Edit(Edit),
    Import(Import),
    Init(Init),
    Query(Query),
    Remove(Remove),
}

/// Add a new directory or increment its rank
#[derive(Debug, Parser)]
#[clap(
    author,
    help_template = HelpTemplate,
)]
pub struct Add {
    #[clap(num_args = 1.., required = true, value_hint = ValueHint::DirPath)]
    pub paths: Vec<PathBuf>,

    /// The rank to increment the entry if it exists or initialize it with if it
    /// doesn't
    #[clap(short, long)]
    pub score: Option<f64>,
}

/// Edit the database
#[derive(Debug, Parser)]
#[clap(
    author,
    help_template = HelpTemplate,
)]
pub struct Edit {
    #[clap(subcommand)]
    pub cmd: Option<EditCommand>,
}

#[derive(Clone, Debug, Subcommand)]
pub enum EditCommand {
    #[clap(hide = true)]
    Decrement { path: String },
    #[clap(hide = true)]
    Delete { path: String },
    #[clap(hide = true)]
    Increment { path: String },
    #[clap(hide = true)]
    Reload,
}

/// Import entries from another application
#[derive(Debug, Parser)]
#[clap(
    author,
    help_template = HelpTemplate,
)]
pub struct Import {
    #[clap(value_hint = ValueHint::FilePath)]
    pub path: PathBuf,

    /// Application to import from
    #[clap(value_enum, long)]
    pub from: ImportFrom,

    /// Merge into existing database
    #[clap(long)]
    pub merge: bool,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum ImportFrom {
    Autojump,
    #[clap(alias = "fasd")]
    Z,
}

/// Generate shell configuration
#[derive(Debug, Parser)]
#[clap(
    author,
    help_template = HelpTemplate,
)]
pub struct Init {
    #[clap(value_enum)]
    pub shell: InitShell,

    /// Prevents querypath from defining the `z` and `zi` commands
    #[clap(long, alias = "no-aliases")]
    pub no_cmd: bool,

    /// Changes the prefix of the `z` and `zi` commands
    #[clap(long, default_value = "z")]
    pub cmd: String,

    /// Changes how often querypath increments a directory's score
    #[clap(value_enum, long, default_value = "pwd")]
    pub hook: InitHook,
}

#[derive(ValueEnum, Clone, Copy, Debug, Eq, PartialEq)]
pub enum InitHook {
    None,
    Prompt,
    Pwd,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum InitShell {
    Bash,
    Elvish,
    Fish,
    Nushell,
    #[clap(alias = "ksh")]
    Posix,
    Powershell,
    Tcsh,
    Xonsh,
    Zsh,
}

/// Search for a directory in the database
#[derive(Debug, Parser)]
#[clap(
    author,
    help_template = HelpTemplate,
)]
pub struct Query {
    pub keywords: Vec<String>,

    /// Show unavailable directories
    #[clap(long, short)]
    pub all: bool,

    /// Use interactive selection
    #[clap(long, short, conflicts_with = "list")]
    pub interactive: bool,

    /// List all matching directories
    #[clap(long, short, conflicts_with = "interactive")]
    pub list: bool,

    /// Print score with results
    #[clap(long, short)]
    pub score: bool,

    /// Exclude the current directory
    #[clap(long, value_hint = ValueHint::DirPath, value_name = "path")]
    pub exclude: Option<String>,
}

/// Remove a directory from the database
#[derive(Debug, Parser)]
#[clap(
    author,
    help_template = HelpTemplate,
)]
pub struct Remove {
    #[clap(value_hint = ValueHint::DirPath)]
    pub paths: Vec<String>,
}
