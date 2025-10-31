# ReaSoning
基于反向链接算法的命令行推理程序。
## 用法
```shell
Usage: reasoning [OPTIONS] <KNOWLEDGE_BASE> <STATEMENT>

Arguments:
  <KNOWLEDGE_BASE>  用于推理的已知条件知识库
  <STATEMENT>       待证命题

Options:
      --kbfile   为知识库传入JSON文件
      --file     为待证命题传入JSON文件
      --verbose  显示详细证明过程
  -h, --help     Print help
```
其中知识库和待证命题格式参考`examples`下示例。
