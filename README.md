<div align="center">
  👩🏽‍💻🎒
</div>

<h1 align="center">
  devtogo
</h1>

<p align="center">
    a dev.to tool for the road
</p>

<div align="center">
  <a alt="GitHub Actions" href="https://github.com/softprops/devtogo/actions">
    <img src="https://github.com/softprops/devtogo/workflows/Main/badge.svg"/>
  </a>
  <a alt="crates.io" href="https://crates.io/crates/devtogo">
    <img src="https://img.shields.io/crates/v/devtogo.svg?logo=rust"/>
  </a>
  <a alt="license" href="LICENSE">
    <img src="https://img.shields.io/badge/license-MIT-brightgreen.svg"/>
  </a>
</div>

<br />

## install


### Via homebrew (osx)

```sh
$ brew install softprops/tools/devtogo
```

### Via cargo

```sh
$ cargo install devtogo
```

### Via GitHub Releases

Using a version from the GitHub [Releases Page](https://github.com/softprops/devtogo/releases),
substitute VERSION below

```sh
$ cd $HOME/bin
$ VERSION=v0.1.0 curl -L "https://github.com/softprops/devtogo/releases/download/${VERSION}/devtogo-$(uname -s)-$(uname -m).tar.gz" \
  | tar -xz -C ~/bin
```

## usage

devtogo is a cli primarily focused on publishing offline markdown files to [dev.to](https://dev.to/). These files follow the same formatting rules documented in [dev.to's editor guide](https://dev.to/p/editor_guide).

At a bare minimum you'll want to declare a title in a frontmatter section of your markdown file.

```md
---
title: my very first post
---
# hello everybody
```


To get started, you will first need to export an `DEVTO_API_KEY` env variable. You can get one [here](https://dev.to/settings/account)

The most basic usage is to run the program inside the directory containing content

```sh
$ devtogo
```

This will scan the current working directory for articles: markdown documents containing frontmatter describing metadata about the article. devtogo uses the `title` frontmatter field as a unique identifier to compare existing remote content. 

When it can't resolve an article by title it uploads it. When it can, it compares content and uploads local copy if the content of the local copy differs.

> you can use the `published` frontmatter to indicate if and when an article should be published
  by default articles are saved as drafts only you can see. Setting published to true will publish articles.
  If you do this by accident you can set published back to false to unpublish an article if needed

To be more explicit you can provide a source argument which provides a path where content
is stored.

```sh
$ devtogo --source path/to/content
```

You can also experiment without actually posting your content using the `--dryrun` flag. This will perform all operations
except for uploading your content. This may be useful for validating your content.

```sh
$ devto --dryrun
```

You can always review the posts uploaded in [your dev.to dashboard online](https://dev.to/dashboard)

Doug Tangren (softprops) 2020