# Crafting Bundler
[TOC]
## Core Concepts
### Compiler & Compilation
一般的 Bundler 都提供了两种编译模式，Build 和 Watch 模式，其中 Build 模式主要用于生产环境构建，一般编译完进程或者任务就终止，而 Watch 模式则会继续监听用户项目的变动，项目变动后会触发新的编译，Watch 还有一种更高级的模式既 Reload 模式，其不仅仅会重新编译产物，还会触发新的产物重新执行（这里的执行包括在 浏览器和 Node等环境下的重新执行），大部分 Bundler 都会提供两个子命令或者参数区分这两种模式，Bundler 通常需要创建一个编译上下文来执行编译，这个上下文包括用户的配置和宿主的执行环境(如FS)，这些上下文在整个编译过程中通常是不变的，在 Webpack 下这个实例称为 Compiler，同时在 Watch 模式下，每次修改还需要创建一个独立新的编译上下文实例，这里需要一个独立的示例是为了避免多个编译同时存在互相干扰，这个编译实例和当前的项目内容绑定包含项目相关的一些数据结构，如 Module Graph、Chunk Graph等，这个实例在 Webpack 中称为 Compilation.


### Module & Dependency
Module 和 Dependency 是 Bundler 中最为核心的概念，其贯穿了 Bundler 构建流程的始终,其 也是 Webpack 核心架构最为关键和最为复杂的概念。
#### Module
##### First Class Module
Webpack5 中一个最被忽视的架构设计变动就是支持了更多的一等公民，CSS、Asset等常用的类型不再需要转换成 JavaScript 才能被Bundler识别，这提供了更多的优化的可能性。

##### TypeScript ? The Good The Bad The Ugly Parts
##### Normal Module & Context Module
##### Module Factory
##### Module Resolution (Resolver)
##### Virtual Module
###### Monorepo Support
##### Parser
##### Loader


#### Dependency 
Webpack 中的 Dependency 是一个复杂的概念，其同时承载了多种功能
#### Chunk
#### Code Splitting
#### Bundle Splitting

### Code Generation
#### Parse & Generator
#### ESM vs Custom Runtime
#### ESM & CJS Interop
#### Runtime

## Core Implementation
### Scan
### Link
### Plugin Design

#### Transform & Filter & Module Type

#### Loader vs Transform hook

#### AST Reuse

### HMR
#### Live Reload & Hot Module Reload(HMR) & Fast-Refresh

#### Incremental Build
### Code Splitting & Bundle Splitting

### Tree Shaking

### Scope Hoisting

### Layer & RSC

### Module Federation

