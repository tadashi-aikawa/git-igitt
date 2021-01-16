# git-igitt

[![Build Status](https://travis-ci.com/mlange-42/git-igitt.svg?branch=master)](https://travis-ci.com/mlange-42/git-igitt)

Interactive Git terminal application to browse and visualize Git history graphs in a comprehensible way, following different branching models.

![git-igitt-animation](https://user-images.githubusercontent.com/44003176/104823331-af30cd00-5849-11eb-97b5-1bea6efc133c.gif)

**git-igitt** is based on [git-graph](https://github.com/mlange-42/git-graph). The image below shows an example using the [GitFlow](https://nvie.com/posts/a-successful-git-branching-model/) branching model for a comparison between graphs generated by git-graph (far left) versus other tools and Git clients. 

> GitFlow was chosen for its complexity, while any other branching model is supported, including user-defined ones.

![Graph comparison between tools](https://user-images.githubusercontent.com/44003176/103466403-36a81780-4d45-11eb-90cc-167d210d7a52.png)

Decide for yourself which graph is the most comprehensible. :sunglasses:

:warning: **This project is still in a very early stage. For reporting issues, see [here](#contributing)** :warning:

## Features

* View structured graphs directly in the terminal
* Interactively browse the Git history and view commits and diffs
* Pre-defined and custom branching models and coloring
* View old and new file versions without checking them out, with syntax highlighting!

## Installation

**Pre-compiled binaries**

1. Download the [latest binaries](https://github.com/mlange-42/git-igitt/releases) for your platform
2. Unzip somewhere
3. *Optional:* add directory `git-igitt` to your `PATH` environmental variable

**Using `cargo`**

In case you have [Rust](https://www.rust-lang.org/) installed, you can install with `cargo`:

```
cargo install --git https://github.com/mlange-42/git-igitt
```

## Usage

For basic usage, run the following command:

```
git-igitt
```

> Note: git-graph needs to be on the PATH, or you need use the full path to git-graph:
> 
> ```
> C:/path/to/git-igitt/git-igitt
> ```

If git-igitt is started inside a Git repository's folder, this repository will be displayed. Otherwise, a file dialog will appear that let's you select a repository.

**Branching models**

By default, git-igitt assumes the `git-graph` branching model. To change the branching model in the application, press `M`. You can then set the model for the current session with `Enter`, or permanently for the repository with `P`. 

Alternatively, start git-graph with a specific model, e.g. `simple`:

```
git-igitt --model simple
```

Or set the model for the repository in the current path permanently:

```
git-igitt model simple
```

**Get help**

To view key bindings and help in the application, press `H` or `F1`. 

For the full CLI help describing all options, use:

```
git-igitt -h
git-igitt --help
```

For details on **branching models**, **styles** and commit **formatting**, see the [git-graph manual](https://github.com/mlange-42/git-graph/blob/master/docs/manual.md).

## Custom branching models

Branching models are configured using the files in `APP_DATA/git-graph/models` (git-igitt shares these files with [git-graph](https://github.com/mlange-42/git-graph)).

* Windows: `C:\Users\<user>\AppData\Roaming\git-graph`
* Linux: `~/.config/git-graph`
* OSX: `~/Library/Application Support/git-graph`

File names of any `.toml` files in the `models` directory can be selected in the application or used with parameter `--model`, or via sub-command `model`. E.g., to start with a branching model defined in `my-model.toml`, use:

```
git-igitt --model my-model
```

For details on **how to create your own branching models** see section 
[Custom branching models](https://github.com/mlange-42/git-graph/blob/master/docs/manual.md#custom-branching-models) of the git-graph manual.

## Limitations

* Summaries of merge commits (i.e. 1st line of message) should not be modified! git-graph needs them to categorize merged branches.
* Currently, the history can only be browsed. So far, no Git commands are implemented.
* Supports only the primary remote repository `origin`.
* Does currently not support "octopus merges" (i.e. no more than 2 parents)
* Syntax highlighting may be slow for large files (turn on/off with by typing `S`).

## Contributing

Please report any issues and feature requests in the [issue tracker](https://github.com/mlange-42/git-igitt/issues).

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.
