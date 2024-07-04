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
