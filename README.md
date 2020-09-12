# Gitfindr-rs

This is my first Rust project that will probably end up in a usable state.

## What it does
This is a CLI tool that can help you keep track of git repositories on your computer. You can add, remove, show info and list repositories

## How to run
1. Make sure you have Rust 1.46 installed.
2. Clone this repository and build the executable.
3. Use the executable!

## Commands
- `gitfindr.exe add -alias [repo-name] -path [path-to-repo]` to add the given repo to the list of tracked repos.
- `gitfindr.exe add -alias [repo-name] -d [path-to-dir]` to add all repos inside the given dir to list of tracked repos.
- `gitfindr.exe list` to list all currently tracked repos.
- `gitfindr.exe show -n [repo-name]` to show info on the given repo.
- `gitfindr.exe remove -n [repo-name` to remove the given repo from the list of tracked repos.