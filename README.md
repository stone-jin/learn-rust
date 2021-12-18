# learn-rust

## 一、使用rust开发一个cli工具

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

## 二、rust和c语言互相调用

目录：ffi_lib

- [x] 构建一个简单的C语言
- [x] 构建一个rust程序，生成一个方法给c语言调用
- [ ] rust返回结果给c语言
- [ ] c语言给一个参数给rust函数 

使用方法：

```bash
cd ffi_lib
make build
```

输出结果如下：

```bash
hello from rust!
hello world
```

### 三、使用napi.rs和js相互交互

目录：napi

- [x] 创建一个napi.rs项目
- [x] 用napi.rs构建一个npm包
- [x] 测试代码：test/index.js，测试async await函数和普通函数
- [] 发布npm包