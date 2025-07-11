<!-- markdownlint-disable-file MD024 -->

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.9.8] - 2025-05-27

### Added

- Support for Tcsh.
- Added `--score` flag to `querypath add`.
- POSIX: add doctor to diagnose common issues.
- Nushell: add CLI completions.

### Changed

- Bash: querypath will now automatically `cd` when selecting Space-Tab completions.

### Fixed

- Bash: doctor now handles `PROMPT_COMMAND` being an array.
- Bash: doctor now handles Visual Studio Code's shell integration.
- Bash: completions now work with `ble.sh`.
- Nushell: stop ignoring symlinks when `cd`-ing into a directory.
- Fzf: updated minimum supported version to v0.51.0.
- PowerShell: avoid setting `$error` when defining `__querypath_hooked`.
- PowerShell: handle special characters in file paths when `cd`-ing into them.
- Database corruption issue when the filesystem is 100% full.

## [0.9.7] - 2025-02-10

### Added

- Nushell: support for 0.102.0.
- Bash / Zsh: add doctor to diagnose common issues.

### Fixed

- ksh: alias to regular POSIX implementation for better compatibility.

## [0.9.6] - 2024-09-19

### Fixed

- Fish: `builtin abbr` doesn't work on older versions.
- Zsh: make `__querypath_z_complete` available with `--no-cmd`.

## [0.9.5] - 2024-09-13

### Added

- Zsh: improved `cd` completions.
- Lazily delete excluded directories from the database.
- Fish: detect infinite loop when using `alias cd=z`.
- Installer: added flags for `--bin-dir`, `--man-dir`, `--arch`, and `--sudo`.
- Nushell: support for v0.94.0+.
- Bash/Fish/Zsh: support for `z -- dir` style queries.
- Fish: improved Space-Tab completions.
- Ksh: added support for the Korn shell.

### Changed

- fzf: removed `--select-1` from default options. The interactive selector will
  now open up even if there is only one match.
- Enforce that `$_QP_DATA_DIR` is an absolute path.

### Fixed

- Zsh: Space-Tab completion repeating output multiple times when matching single
  directory
- Fish / Nushell / PowerShell: handle queries that look like args (e.g. `z -x`).
- Elvish: `z -` now works as expected.
- Fish: generated shell code avoids using aliased builtins.
- Fish: `cd` command is now copied directly from
  `$__fish_data_dir/functions/cd.fish`. This should minimize the chances of an
  infinite loop when aliasing `cd=z`.
- Symlinks not getting added to the database when `$_QP_RESOLVE_SYMLINKS=0`.
- Symlinked database files getting replaced instead of the actual files.

## [0.9.4] - 2024-02-21

### Changed

- Zsh: improved Space-Tab completions.

## [0.9.3] - 2024-02-13

### Added

- Nushell: support for v0.89.0.

## [0.9.2] - 2023-08-04

### Added

- Short option `-a` for `querypath query --all`.

### Fixed

- PowerShell: use `global` scope for variables / functions.

## [0.9.1] - 2023-05-07

### Added

- Fish/Zsh: aliases on `__querypath_z` will now use completions.
- Nushell: support for v0.78.0.
- Fish: plugin now works on older versions.
- PowerShell: warn when PowerShell version is too old for `z -` and `z +`.
- PowerShell: support for PWD hooks on all versions.

### Fixed

- Fish: not providing `cd` completions when there is a space in the path.
- Bash/Fish/Zsh: providing `z` completions when the last argument starts with `z!`.
- Bash/Fish/Zsh: attempting to `cd` when the last argument is `z!`.

## [0.9.0] - 2023-01-08

### Added

- `edit` subcommand to adjust the scores of entries.

### Fixed

- Zsh: completions clashing with `zsh-autocomplete`.
- Fzf: 'invalid option' on macOS.
- PowerShell: handle UTF-8 encoding correctly.
- Zsh: don't hide output from `chpwd` hooks.
- Nushell: upgrade minimum supported version to v0.73.0.
- Zsh: fix extra space in interactive completions when no match is found.
- Fzf: various improvements, upgrade minimum supported version to v0.33.0.
- Nushell: accidental redefinition of hooks when initialized twice.

### Removed

- `remove -i` subcommand: use `edit` instead.

## [0.8.3] - 2022-09-02

### Added

- Nushell: support for `z -`.
- Nushell: support for PWD hooks.

### Changed

- Fish: change fuzzy completion prefix to `z!`.
- Zsh: allow `z` to navigate dirstack via `+n` and `-n`.
- Fzf: improved preview window.

### Fixed

- Bash: double forward slash in completions.

## [0.8.2] - 2022-06-26

### Changed

- Fzf: show preview window below results.

### Fixed

- Bash/Fish/POSIX/Zsh: paths on Cygwin.
- Fish: completions not working on certain systems.
- Bash: completions not escaping spaces correctly.

## [0.8.1] - 2021-04-23

### Changed

- Manpages: moved to `man/man1/*.1`.
- Replace `--no-aliases` with `--no-cmd`.
- Elvish: upgrade minimum supported version to v0.18.0.
- Nushell: upgrade minimum supported version to v0.61.0.

### Fixed

- Bash/Zsh: rename `_z` completion function to avoid conflicts with other shell
  plugins.
- Fzf: added `--keep-right` option by default, upgrade minimum supported version
  to v0.21.0.
- Bash: only enable completions on 4.4+.
- Fzf: bypass `ls` alias in preview window.
- Retain ownership of database file.
- `querypath query --interactive` should not conflict with `--score`.

## [0.8.0] - 2021-12-25

### Added

- Zsh: completions for `z` command.

### Changed

- Fzf: better default options.
- Fish: interactive completions are only triggered when the last argument is
  empty.
- PowerShell: installation instructions.

### Fixed

- PowerShell: use global scope for aliases.
- Zsh: fix errors with `set -eu`.
- Fzf: handle early selection.
- PowerShell: correctly handle escape characters in paths.
- Parse error on Cygwin/MSYS due to CRLF line endings.
- Fzf: handle spaces correctly in preview window.
- Bash: avoid initializing completions on older versions.
- Fzf: avoid launching binary from current directory on Windows.

## [0.7.9] - 2021-11-02

### Changed

- Bash/Fish: improved completions for `z` command.

### Fixed

- Fish: error erasing completions on older versions.
- PowerShell: enable `--cmd cd` to replace the `cd` command.

## [0.7.8] - 2021-10-21

### Added

- Auto-generated completions for [Fig](https://fig.io/).

### Fixed

- Compile error with `clap v3.0.0-beta.5`.

## [0.7.7] - 2021-10-15

### Fixed

- PowerShell: hook not initializing correctly.

## [0.7.6] - 2021-10-13

### Changed

- Nushell: upgrade minimum supported version to v0.37.0.

### Fixed

- Xonsh: error messages in `zi`.
- Xonsh: configuration environment variables not being handled correctly.

## [0.7.5] - 2021-09-09

### Added

- Bash/Elvish: completions for `z` command.

### Changed

- Nushell: upgrade minimum supported version to v0.36.0.
- Nushell: easier installation instructions.

### Fixed

- Elvish: unable to `z` into directories by path.
- Elvish: don't show traceback when `z` or `zi` fails.
- Elvish: nested shells do not initialize correctly.

## [0.7.4] - 2021-08-15

### Fixed

- Compile error with `clap v3.0.0-beta.4`.

## [0.7.3] - 2021-08-05

### Added

- `querypath add` and `querypath remove` now accept multiple arguments.

### Fixed

- Nushell: errors on 0.33.0.
- PowerShell: errors when initializing in `StrictMode`.
- Bash/POSIX: remove conflicting alias definitions when initializing.
- Bash: remove extra semicolon when setting `$PROMPT_COMMAND`.
- Xonsh: use shell environment instead of `os.environ`.

## [0.7.2] - 2021-06-10

### Fixed

- `querypath -V` not printing version.

## [0.7.1] - 2021-06-09

### Added

- Auto-generated shell completions.
- `querypath query --all` for listing deleted directories.
- Lazy deletion for removed directories that have not been accessed in > 90
  days.
- Nushell: support for 0.32.0+.

### Fixed

- Nushell: avoid calling `__querypath_hook` on non-filesystem subshells.
- Fish: `alias cd=z` now works, but it must be done after calling `querypath init`.
- PowerShell: avoid calling `__querypath_hook` on non-filesystem providers.
- Fish: avoid calling `__querypath_hook` in private mode.

## [0.7.0] - 2021-05-02

### Added

- Manpages for all subcommands.
- Default prompt for Nushell.

### Changed

- `querypath remove -i` now accepts multiple selections.
- `querypath add` no longer accepts zero parameters.
- `$_QP_EXCLUDE_DIRS` now defaults to `"$HOME"`.
- Binary releases now use `.zip` on Windows, `.tar.gz` otherwise.

### Fixed

- `cd -` on Fish shells.
- `__querypath_hook` no longer changes value of `$?` within `$PROMPT_COMMAND` on
  Bash.

### Removed

- GitHub install script.
- Release binaries built with `glibc`, use `musl` instead.

## [0.6.0] - 2021-04-09

### Added

- Support for [Nushell](https://www.nushell.sh/).
- Support for [Elvish](https://elv.sh/).

### Changed

- `z` now excludes the current directory from search results.

### Fixed

- Removed backtraces on Rust nightly.
- Generated shell code avoids using aliased builtins.
- Handle broken pipe errors gracefully when writing to streams.
- NUL file appearing in working directory on Windows.
- Accidental redefinition of hooks when initialized twice on some shells.
- querypath unable to find itself on Xonsh shells.

### Removed

- Aliases: `za`, `zq`, `zqi`, `zr`, `zri`. These are trivial aliases that can
  easily be defined manually, and aren't very useful to most users.

## [0.5.0] - 2020-10-30

### Added

- `$_QP_EXCLUDE_DIRS` now supports globs.
- `querypath init` now defines `__querypath_z*` functions that can be aliased as
  needed.
- Support for the [Xonsh](https://xon.sh/) shell.
- `querypath import` can now import from Autojump.

### Changed

- `querypath init --no-aliases` no longer generates `z` or `zi`.

### Fixed

- Clobber conflicting alias definitions in Bash/Fish/Zsh/POSIX shells.

### Removed

- Deprecated PWD hooks for POSIX shells.
- Lazy deletion for inaccessible directories.

## [0.4.3] - 2020-07-04

### Fixed

- Bug in Fish init script.

## [0.4.2] - 2020-07-03

### Added

- `$_QP_FZF_OPTS` to specify custom options for `fzf`
- `querypath query --list` to list all matches
- `querypath query --score` to show score along with result

### Changed

- Increased default value of `$_QP_MAXAGE` to `10000`.
- Symlinks are treated as separate directories by default, this can be changed
  by setting `_QP_RESOLVE_SYMLINKS=1`.

### Removed

- Help menus for `z` and `zri`.
- `querypath remove -i` is replaced with `zri`.

## [0.4.1] - 2020-05-25

### Added

- Support for PowerShell.

### Removed

- Backward compatibility with `v0.2.x` databases.
- Support for paths with invalid UTF-8.

## [0.4.0] - 2020-05-03

### Added

- Interactive mode for removing entries (`querypath remove -i`).
- Aliases for interactive `query` and `remove` (`zqi` and `zri` respectively).
- PWD hooks for POSIX shells.

### Changed

- `querypath remove` now throws an error if there was no match in the database.
- Interactive mode in `querypath` no longer errors out if `fzf` exits gracefully.
- Canonicalize to regular paths instead of UNC paths on Windows.
- `querypath init` now uses PWD hooks by default for better performance.
- `$_QP_ECHO` now only works when set to `1`.
- Using the `--z-cmd` flag now also renames the associated aliases.
- The `--z-cmd` flag has been renamed to `--cmd`.
- The `--no-define-aliases` flag has been renamed to `--no-aliases`.

### Fixed

- Fish no longer `cd`s to the user's home when no match is found.

## [0.3.1] - 2020-04-03

### Added

- Version output displays `git` revision information.
- `--z-cmd` flag for `querypath init` to rename the `z` command to something else.

### Changed

- `querypath query` output no longer has the `query:` prefix.

### Fixed

- Queries now also include checks for if the top level directory matches.

## [0.3.0] - 2020-03-30

### Added

- Automatic migration from `v0.2.x` databases.
- `$_QP_EXCLUDE_DIRS` to prevent directories from being added to the database.
- Support for POSIX-compliant shells.

### Changed

- Database location defaults to user's local data directory.
- Database schema now includes a version number.
- `migrate` subcommand renamed to `import`.

### Fixed

- Thread safety using unique tempfile names for each `querypath` instance.
- Incomprehensive "could not allocate" message on database corruption.

## [0.2.2] - 2020-03-20

### Fixed

- Incorrect exit codes in `z` command on Fish.

### Removed

- File locks on database.

## [0.2.1] - 2020-03-16

### Added

- `$_QP_ECHO` to echo match before `cd`ing.
- Minimal `ranger` plugin.
- PWD hook to only update the database when the current directory is changed.
- Support for Bash.
- `migrate` subcommand to allow users to migrate from `z`.

### Fixed

- Interactive queries causing other open shells to hang.

## [0.2.0] - 2020-03-11

### Added

- `init` subcommand to remove dependency on shell plugin managers.
- Support for `z -` command to go to previous directory.
- `Cargo.lock` for more reproducible builds.
- Support for the Fish shell.

### Fixed

- `_querypath_precmd` overriding other precmd hooks on Zsh.

## [0.1.1] - 2020-03-08

### Added

- Install script for Linux/macOS users.
- Aging algorithm to remove stale entries.

### Changed

- Database schema now uses `f64` values for rank instead of `i32`.

### Fixed

- Multiple hooks being added upon initializing `querypath` multiple times.

## [0.1.0] - 2020-03-05

### Added

- GitHub Actions pipeline to build and upload releases.
- Add support for Zsh.

[0.9.8]: https://github.com/ajeetdsouza/querypath/compare/v0.9.7...v0.9.8
[0.9.7]: https://github.com/ajeetdsouza/querypath/compare/v0.9.6...v0.9.7
[0.9.6]: https://github.com/ajeetdsouza/querypath/compare/v0.9.5...v0.9.6
[0.9.5]: https://github.com/ajeetdsouza/querypath/compare/v0.9.4...v0.9.5
[0.9.4]: https://github.com/ajeetdsouza/querypath/compare/v0.9.3...v0.9.4
[0.9.3]: https://github.com/ajeetdsouza/querypath/compare/v0.9.2...v0.9.3
[0.9.2]: https://github.com/ajeetdsouza/querypath/compare/v0.9.1...v0.9.2
[0.9.1]: https://github.com/ajeetdsouza/querypath/compare/v0.9.0...v0.9.1
[0.9.0]: https://github.com/ajeetdsouza/querypath/compare/v0.8.3...v0.9.0
[0.8.3]: https://github.com/ajeetdsouza/querypath/compare/v0.8.2...v0.8.3
[0.8.2]: https://github.com/ajeetdsouza/querypath/compare/v0.8.1...v0.8.2
[0.8.1]: https://github.com/ajeetdsouza/querypath/compare/v0.8.0...v0.8.1
[0.8.0]: https://github.com/ajeetdsouza/querypath/compare/v0.7.9...v0.8.0
[0.7.9]: https://github.com/ajeetdsouza/querypath/compare/v0.7.8...v0.7.9
[0.7.8]: https://github.com/ajeetdsouza/querypath/compare/v0.7.7...v0.7.8
[0.7.7]: https://github.com/ajeetdsouza/querypath/compare/v0.7.6...v0.7.7
[0.7.6]: https://github.com/ajeetdsouza/querypath/compare/v0.7.5...v0.7.6
[0.7.5]: https://github.com/ajeetdsouza/querypath/compare/v0.7.4...v0.7.5
[0.7.4]: https://github.com/ajeetdsouza/querypath/compare/v0.7.3...v0.7.4
[0.7.3]: https://github.com/ajeetdsouza/querypath/compare/v0.7.2...v0.7.3
[0.7.2]: https://github.com/ajeetdsouza/querypath/compare/v0.7.1...v0.7.2
[0.7.1]: https://github.com/ajeetdsouza/querypath/compare/v0.7.0...v0.7.1
[0.7.0]: https://github.com/ajeetdsouza/querypath/compare/v0.6.0...v0.7.0
[0.6.0]: https://github.com/ajeetdsouza/querypath/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/ajeetdsouza/querypath/compare/v0.4.3...v0.5.0
[0.4.3]: https://github.com/ajeetdsouza/querypath/compare/v0.4.2...v0.4.3
[0.4.2]: https://github.com/ajeetdsouza/querypath/compare/v0.4.1...v0.4.2
[0.4.1]: https://github.com/ajeetdsouza/querypath/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/ajeetdsouza/querypath/compare/v0.3.1...v0.4.0
[0.3.1]: https://github.com/ajeetdsouza/querypath/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/ajeetdsouza/querypath/compare/v0.2.2...v0.3.0
[0.2.2]: https://github.com/ajeetdsouza/querypath/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/ajeetdsouza/querypath/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/ajeetdsouza/querypath/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/ajeetdsouza/querypath/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/ajeetdsouza/querypath/commits/v0.1.0
