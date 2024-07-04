# Contributing

This guide outlines the process for creating a new source extension for Midoku.
Please **read it carefully** if you are a new contributor or do not have any
experience on the required languages and tooling.

This guide is not definitive and may change over time. If you find any issues
on it, feel free to [open an issue][new-issue] or a pull request.

[new-issue]: https://github.com/sehnryr/midoku-community-extensions/issues/new

## Table of Contents

1. [Prerequisites](#prerequisites)
    1. [Tools](#tools)
    2. [Cloning the repository](#cloning-the-repository)
2. [Getting help](#getting-help)
3. [Writing an extension](#writing-an-extension)
    1. [File structure](#file-structure)
    2. [Exported functions](#exported-functions)
4. [Testing](#testing)
    1. [Unit tests](#unit-tests)
    2. [Integration tests](#integration-tests)
5. [Building](#building)

## Prerequisites

Before starting, please note that the ability to use the following technologies
is **required** and that existing contributors will not actively teach them to
you.

- [Rust-lang](https://www.rust-lang.org/)
- Web scraping
    - [HTML](https://developer.mozilla.org/en-US/docs/Web/HTML)
    - [CSS](https://developer.mozilla.org/en-US/docs/Web/CSS)
- [Git](https://git-scm.com/)

### Tools

- [cargo-component](https://github.com/bytecodealliance/cargo-component)
- Basic image editing software

### Cloning the repository

The following steps can be used to skip unrelated extensions and branches, which
will make it faster to pull and navigate. This will also reduce disk usage and
network traffic.

These steps are optional and only needed when the repository is too large and
contains a lot of extensions.

0. When forking a repository, only fork the `main` branch. You may also want to
    disable GitHub Actions on your fork.

1. Do a partial clone:

    ```sh
    git clone --filter=blob:none --sparse <fork-repo-url>
    cd midoku-community-extensions/
    ```

2. Configure spase checkout:

    Enable it using the following command:

    ```sh
    git sparse-checkout set
    ```

    Edit `.git/info/sparse-checkout` and add the extensions you want to work on:

    ```.gitignore
    /*
    !/src

    # Add the extensions you want to work on
    /src/<lang>/<extension-name>
    ```

    The syntax is the same as `.gitignore` files. Here we first add everything
    to the sparse checkout, then exclude the `src` directory and finally include
    the extensions we want to work on.

3. Configure remotes:

    ```sh
    # add upstream
    git remote add upstream https://github.com/sehnryr/midoku-community-extensions
    # optionally disable push to upstream
    git remote set-url --push upstream no_pushing
    # optionally fetch main only (ignore all other branches)
    git config remote.upstream.fetch "+refs/heads/main:refs/remotes/upstream/main"
    # update remotes
    git remote update
    # track main of upstream instead of fork
    git branch main -u upstream/main
    ```

4. Useful configurations (optional):

    ```sh
    # prune obsolete remote branches on fetch
    git config remote.origin.prune true
    # fast-forward only when pulling main branch
    git config pull.ff only
    ```

> [!IMPORTANT]
> Later, if changes are made to the sparse checkout filter, you will need to
> reapply it using `git sparse-checkout reapply`.

Read more on [Git's object model][git-object-model],
[partial clone][git-partial-clone], [sparse checkout][git-sparse-checkout] and
[negative refspecs][git-negative-refspecs].

[git-object-model]: https://github.blog/2020-12-17-commits-are-snapshots-not-diffs/
[git-partial-clone]: https://github.blog/2020-12-21-get-up-to-speed-with-partial-clone-and-shallow-clone/
[git-sparse-checkout]: https://github.blog/2020-01-17-bring-your-monorepo-down-to-size-with-sparse-checkout/
[git-negative-refspecs]: https://github.blog/2020-10-19-git-2-29-released/#user-content-negative-refspecs

## Getting help

Join the [Midoku Discord server][discord-invite] for online help and to ask
questions while developing your extension. When doing so, please ask it in the
[#extension-dev][discord-extension-dev] channel.

[discord-invite]: https://discord.gg/SDSeUdfC33
[discord-extension-dev]: https://discord.com/channels/1258440323886219394/1258449253399789708

Some features and tricks not covered in this guide can be found in the existing
extension code. Please refer to it for examples.

## Writing an extension

The quickest way to get started is to copy an existing extension and renaming it
as needed. We also recommend reading through the code of a few existing
extensions before starting.

Each extension should reside in `src/<lang>/<extension-name>`.

`<lang>` should be an [IETF BCP 47][ietf-language-tag] compliant
[language subtag][subtag-registry] (or `multi` for extensions that support
multiple languages). For example, `pt` for Portuguese or Brazilian (`pt-BR`),
`en` for English, etc.

[ietf-language-tag]: https://en.wikipedia.org/wiki/IETF_language_tag
[subtag-registry]: https://www.iana.org/assignments/language-subtag-registry/language-subtag-registry

`<extension-name>` should be a unique name for the extension in kebab-case.

### File structure

The following is the basic file structure for an extension:

```sh
$ tree src/<lang>/<extension-name>
src/<lang>/<extension-name>
├── build.rs
├── Cargo.toml
├── res
│   ├── filters.json
│   ├── icon.png
│   └── source.json
├── src
│   ├── bindings.rs
│   └── lib.rs
└── wit
    ├── deps
    │   ├── midoku-bindings
    │   │   └── bindings.wit
    │   ├── midoku-http
    │   │   └── http.wit
    │   ├── midoku-limiter
    │   │   └── limiter.wit
    │   ├── midoku-settings
    │   │   └── settings.wit
    │   └── midoku-types
    │       └── types.wit
    └── world.wit

10 directories, 13 files
```

`src/bindings.rs` is an automatically generated file by `wit-bindgen` and should
not be edited manually. It contains the bindings to the Wit world.

The `wit` directory contains the Wit world and its dependencies. The `world.wit`
file is the main file that defines the extension's behavior. Its content should
not be edited manually.

`build.rs` is a build script that automatically puts in the version of the
package from `Cargo.toml` into the `res/source.json` file.

#### `res/source.json`

This file contains the manifest of the extension. It should follow this
structure:

```json
{
    "name": "<source-name>",
    "language": "<lang>",
    "version": "<extension-version>",
    "url": "<source-url>",
    "nsfw": <true|false>,
}
```

| Field | Description |
| :--- | :--- |
| `name` | The displayed name of the source. |
| `language` | The language of the source. It should be an [IETF BCP 47][ietf-language-tag] compliant [language tag][subtag-registry]. |
| `version` | The version of the extension. It should follow [Semantic Versioning][semver]. |
| `url` | The URL of the source. |
| `nsfw` | Whether the source contains NSFW content. |

[semver]: https://semver.org/

#### `res/filters.json`

This file contains the search filters for the source. Read through the
`filters.json` of other extensions to understand how to write it.

#### `res/settings.json` (optional)

This file contains the settings for the extension. It contains a dictionary
where the key is the setting unique name and the value is the setting. Read
through the `settings.json` of other extensions to understand how to write it.

#### `res/languages.json` (optional)

If the extension supports multiple languages, this file should be present. It
contains the languages supported by the source. It should follow this structure:

```json
{
    "en_US": {
        "code": "en",
        "default": true
    },
    "ja_JP": {
        "code": "ja"
    },
    ...
}
```

Keys are the [IETF BCP 47][ietf-language-tag] compliant
[language tags][subtag-registry] and values are objects containing the
extension's internal language code and whether it is the default language (only
one language should be marked as default).

#### `src/lib.rs`

This file contains the main logic of the extension. Here is a minimal template:

```rust
#[allow(warnings)]
mod bindings;

use bindings::exports::midoku::bindings::api::Guest;
use bindings::exports::midoku::types::chapter::Chapter;
use bindings::exports::midoku::types::filter::Filter;
use bindings::exports::midoku::types::manga::Manga;
use bindings::exports::midoku::types::page::Page;

struct Component;

impl Guest for Component {
    fn initialize() -> Result<(), ()> {
        ...
    }

    fn get_manga_list(filter: Vec<Filter>, page: u32) -> Result<(Vec<Manga>, bool), ()> {
        ...
    }

    fn get_manga_details(manga_id: String) -> Result<Manga, ()> {
        ...
    }

    fn get_chapter_list(manga_id: String) -> Result<Vec<Chapter>, ()> {
        ...
    }

    fn get_page_list(manga_id: String, chapter_id: String) -> Result<Vec<Page>, ()> {
        ...
    }
}

bindings::export!(Component with_types_in bindings);
```

#### `Cargo.toml`

This file contains the metadata of the extension package. Here is a minimal
template:

```toml
[package]
name = "<extension-package-name>"
version = "0.1.0"
edition.workspace = true
publish.workspace = true

[dependencies]
wit-bindgen-rt = { version = "0.24.0", features = ["bitflags"] }

[build-dependencies]
serde = { version = "1.0.201", features = ["derive"] }
serde_json = "1.0.117"

[lib]
crate-type = ["cdylib"]

[package.metadata.component]
package = "midoku:midoku-extension"

[package.metadata.component.target.dependencies]
"midoku:bindings" = { path = "wit/deps/midoku-bindings" }
"midoku:http" = { path = "wit/deps/midoku-http" }
"midoku:limiter" = { path = "wit/deps/midoku-limiter" }
"midoku:settings" = { path = "wit/deps/midoku-settings" }
"midoku:types" = { path = "wit/deps/midoku-types" }
```

`extension-package-name` should be the name of the extension package. It should
be unique and follow this format: `midoku-<lang>-<source-name>`. For example,
`midoku-multi-mangadex`.

The `package.metadata.component` section is used to specify the extension's
metadata for `cargo-component`. It should not be modified.

The `lib.crate-type` should be set to `["cdylib"]` to build the extension as a
WebAssembly module.

`wit-bindgen-rt` is a dependency that provides the runtime for the Wit bindings.
It is required for all extensions and should not be removed.

Read through the code of other extensions to understand how to write it.

### Exported functions

The following functions should be implemented in the `Guest` trait:

#### `initialize`

This function is called when the extension is initialized. It should be used to
initialize the extension and set up any necessary configuration.

#### `get_manga_list`

This function should return a list of manga based on the given filters and page
number. It should return a tuple containing the list of manga and a boolean
indicating whether there are more pages.

#### `get_manga_details`

This function should return the details of a manga based on the given manga ID.

#### `get_chapter_list`

This function should return a list of chapters based on the given manga ID.

#### `get_page_list`

This function should return a list of pages based on the given manga ID and
chapter ID.

## Testing

To test the extension, run the following command:

```sh
cargo test --package <extension-package-name>
```

Replace `<extension-package-name>` with the name of the extension package.

### Unit tests

Unit tests should be written in the files under the `src` directory. The tests
should be placed in the same file as the code they are testing.

### Integration tests

Integration tests should be written in the `tests` directory. Each test file
should be named `<test-name>.rs` and should contain the tests for the
extension.

Read through the [integration tests](src/multi/mangadex/tests/integration.rs) of
MangaDex's extension to understand how to write them.

## Building

To build the extension, run the following command:

```sh
cargo component build --release --target wasm32-unknown-unknown --package <extension-package-name>
```

Replace `<extension-package-name>` with the name of the extension package.

You can also build all extensions (which is not recommended when the number of
extensions is large) by running the following command:

```sh
cargo component build --release --target wasm32-unknown-unknown --workspace
```

The built extension will be located in the
`target/wasm32-unknown-unknown/release` directory.
