# unpack (Crafting Bundler)
Crafting Bundler builds Unplugin API on top of Webpack architecture.

This is the repo used for the in-progress book "Crafting Bundler". It contains the markdown text of the book, full implementation of bundler(unpack).

If you find an error or have a suggestion, please do file an issue here. Thank you!

Unpack is a miniature model of the bundler, intended to teach the structure of the real bundler.

I realized a small bundler implementation would be useful to others who want to learn how bundler works. So I will rewrite the Rspack in Rust with minimal api(unplugin api),it will contain 1/10 functionalities of the real Rspack bundler. It's intended as a reference in code for people who want to see how bundler actually works without the consideration of performance and huge api.


## Repository Layout
* `book/`- Markdown files for the text of book.
* `.`- Rust implementation of unpack

> [!NOTE]  
> Inspired by https://github.com/sandersn/mini-typescript and https://github.com/codecrafters-io/build-your-own-x and https://github.com/munificent/craftinginterpreters/tree/master
