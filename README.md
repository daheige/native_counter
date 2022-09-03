# native_counter

rust neon project for nodejs ffi call.

# 如何操作
1. 安装好nodejs，查看nodejs版本
```shell
node -v
v16.15.0
```
2. 安装好neon-cli
```shell
npm install neon-cli -g
```
3. 创建native_counter项目
```shell
cd ~
neon new native_counter
```
查看生成的目录
```shell
cd native_counter
tree -L 2 ./
./
├── LICENSE
├── README.md
├── lib
│ └── index.js
├── native
│ ├── Cargo.lock
│ ├── Cargo.toml
│ ├── artifacts.json
│ ├── build.rs
│ ├── index.node
│ ├── src
└── package.json
```
4. 编写rust代码，在native/src/lib.rs添加如下代码：
```shell
use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

fn count_words(mut cx: FunctionContext) ->JsResult<JsNumber>{
    let txt = cx.argument::<JsString>(0)?.value();
    let word = cx.argument::<JsString>(1)?.value();

    // 按照空格进行分割，得到的是一个字符串数组，然后调用filter进行回调处理
    // 统计word出现的次数，强制转换为f64格式，因为js number都是float64类型
    Ok(cx.number(txt.split(" ").filter(|s|{
        s == &word
    }).count() as f64))
}

// 通过宏的方式将函数导出
register_module!(mut cx, {
    cx.export_function("hello", hello)?;
    cx.export_function("count_words", count_words)?;
    Ok(())
});
```
5. 通过neon编译rust生成nodejs 二进制文件index.node
```shell
cd ~/native_counter
neon build
```
这个命令会在native目录下生成index.node

备注：
- nodejs require支持.js,.json,.node结尾的文件加载
- 上线这个rust nodejs拓展，只需要将index.node文件放到native下就可以
6. 在lib/index.js中编写如下代码：
```javascript
const addon = require('../native');

// console.log(addon.hello());

module.exports = {
    hello: addon.hello,
    count_words: addon.count_words
}
```
7. 在native_counter下面编写main.js进行测试
```javascript
const addon = require(".");
let wc = addon.count_words("a test to rust ffi for nodejs test this is test project","test");
console.log("test count:",wc);

// 调用hello函数
console.log(addon.hello());
```
8. 执行node main.js
```shell
node main.js
test count: 3
hello node
```
恭喜你，到这里rust ffi for nodejs call成功运行了！

# neon crate
- https://crates.io/crates/neon
- https://github.com/neon-bindings/neon

# 其他rust ffi for nodejs crate
- https://crates.io/crates/napi
- https://github.com/napi-rs/napi-rs
- https://github.com/napi-rs/napi-rs/tree/main/examples/napi
