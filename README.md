# 项目介绍

## 开发环境设置

### 安装 Rust

* [使用国外源安装](https://www.rust-lang.org/tools/install)

* [使用国内代理源安装](https://rsproxy.cn/#getStarted)
  

### 安装 VSCode 插件

- rust: Rust 官方插件集（里面包含了rust-analyzer, crates, Rust Syntax）
- Even Better TOML: TOML 文件支持
- Better Comments: 优化注释显示
- Error Lens: 错误提示优化
- indent-rainbow: 缩进显示优化
- Prettier - Code formatter: 代码格式化
- REST client: REST API 调试
- Rust Test lens: Rust 测试支持
- Rust Test Explorer: Rust 测试概览
- TODO Highlight: TODO 高亮
- vscode-icons: 图标优化
- YAML: YAML 文件支持
- GitLens: Git 增强
- Github Copilot: 代码提示


### 安装 pre-commit

pre-commit 是一个代码检查工具，可以在提交代码前进行代码检查。

```bash
pipx install pre-commit
```

安装成功后运行 `pre-commit install` 即可。


### 安装 typos

typos 是一个拼写检查工具。

```bash
cargo install typos-cli
```

### 安装 cargo nextest

cargo nextest 是一个 Rust 增强测试工具。

```bash
cargo install cargo-nextest --locked
```
