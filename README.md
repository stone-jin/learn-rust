# learn-rust

## 使用rust开发一个cli工具

目录：cli

- [x] 使用rust编写一个cli工具
  - [x] 读取参数，构建查询结构体
  - [x] 读取文件
  - [x] 过滤结果
  - [x] 输出结果

使用方法：

```bash
cd cli
make build // 或者：cargo run rust ./test/hello.txt
```

输出结果： 读取test文件夹中的 `hello.txt` 里面带有 `rust` 的行

结果如下:
```bash
Welcome to use search Program. Your args is ["target/debug/cli", "hello", "./test/hello.txt"]
query: hello, file_name: ./test/hello.txt
result: 
-> hello world
-> hello rust
```
