# moz-rust-book-club-cli

## Structure of Repo

The structure of this repo is as follows:

- each chapter is a folder
- within each chapter folder, you will find an `author` folder. This is the corresponding folder from the book's [repo](https://github.com/kyclark/command-line-rust/tree/clap_v4#)
- there's an `author's notes` folder with the licenses, README, and utils from the book's repo

## How to Follow Along

To work on a chapter, you should:

1. Go into the respective parent folder. Ex `cd 01_echor`
1. Run `cargo new <name>` at in that parent folder (where `<name>` is the app's name you're building). Ex: `cargo new echor`
1. Rename your newly created folder to your name. Ex `mv echor roux`
1. Go into the folder and do your stuff!

The reason for this is to keep the commands we're using throughout the book the same, but to differentiate everyone's work in the repo

**IMPORTANT NOTE** The CLI book has a 2022 and a 2024 version. The repo's `author` assets have been updated to use the 2024 version of the book. There's significant differences between clap v2 and clap v4. If you're using the 2022 version of the book, please make sure to check out the `main` branch of the book's [repo](https://github.com/kyclark/command-line-rust/tree/main)
