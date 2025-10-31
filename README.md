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
## 仓库文件结构说明
```shell
.
├── benches
│   └── math_benchmark.rs
├── Cargo.lock
├── Cargo.toml
├── examples
│   ├── AIMA
│   │   ├── kb.json
│   │   └── knowledge_base.json
│   └── math
│       ├── math.json
│       ├── math_theorem.json
│       └── res.txt
├── .gitignore
├── LICENSE
├── prolog_ver
│   └── math.pl
├── README.md
└── src
    ├── bc.rs
    ├── bench.rs
    ├── cli.rs
    ├── lib.rs
    ├── main.rs
    └── unify.rs
```
其中：
`benches`文件夹存放性能测试的入口程序。
`examples`文件夹中，`AIMA`文件夹内为源自《人工智能现代方法》的示例知识库与目标命题，`math`文件夹内为源自课程实验要求的数学证明知识库和目标命题。
`prolog_ver`为一个使用`prolog`编写的、带有运行时间测试的证明程序，知识库和目标同`examples/math`。
`src`文件夹中，`bc.rs`为反向链接算法实现；`bench.rs`为性能测试的目标函数，具体内容同`prolog_ver`；`cli.rs`为反向链接算法的命令行包装；`lib.rs`存放了一阶谓词逻辑相关的数据结构，其中包含了变量标准化方法；`main.rs`为命令行程序入口；`unify.rs`实现了合一算法。
