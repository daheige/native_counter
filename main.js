const addon = require(".");
let wc = addon.count_words("a test to rust ffi for nodejs test this is test project","test");
console.log("test count:",wc);

// 调用hello函数
console.log(addon.hello());
console.log("恭喜你，到这里rust ffi for nodejs call成功运行了！");
