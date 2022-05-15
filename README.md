Rubo 集成一些 cli 工具。安装：

```shell
cargo install rubo
```

安装后，通过 `rubo --help` 可查看工具集。

# tree

`rubo tree` 类似于 tree，但更漂亮一些。

# pod

pod 相关工具集：

- `rubo pod dep` 基于 Podfile.lock 分析 pod 依赖
- `rubo pod rdep` 基于 Podfile.lock 分析 pod 的反向依赖