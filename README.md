# moz-rust-book-club-cli

The structure of this repo is as follows:

- each chapter is a folder
- within each chapter folder, you will find an `author` folder. This is the corresponding folder from the book's [repo](https://github.com/kyclark/command-line-rust/tree/clap_v4#)
- there's an `author's notes` folder with the licenses, README, and utils from the book's repo

To work on a chapter, you should:

1. Go into the respective parent folder. Ex `cd 01_echor`
1. Run `cargo new <name>` at in that parent folder (where `<name>` is the app's name you're building). Ex: `cargo new echor`
1. Rename your newly created folder to your name. Ex `mv echor roux`
1. Go into the folder and do your stuff!

The reason for this is to keep the commands we're using throughout the book the same, but to differentiate everyone's work in the repo

