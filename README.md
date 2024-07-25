# rollpack
Unplugin API built on top of Webpack architecture

A miniature model of the bundler, intended to teach the structure of the real bundler.

I realized a small bundler implementation would be useful to others who want to learn how bundler works. So I will rewrite the Rspack in Rust with minimal api(unplugin api),it will contain 1/10 functionalities of the real Rspack bundler. It's intended as a reference in code for people who want to see how bundler actually works without the consideration of performance and huge api.

> [!NOTE]  
> Inspired by https://github.com/sandersn/mini-typescript and https://github.com/codecrafters-io/build-your-own-x
