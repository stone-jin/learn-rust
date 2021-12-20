var hello = require('../');
const path = require('path');

console.log(hello);

console.log(hello.sum(1, 2));

console.log(hello.getNums());

console.log(hello.getStr());

console.log(hello.sumNums([1, 2]));

hello.readFileAsync(path.join(__dirname, './index.js')).then((res) => {
  // console.log(res.toString())
});

hello.asyncMultiTwo(2).then((res) => {
  // console.log(res)
});

let res = hello.bigAdd(BigInt(1000), BigInt(1));
console.log(`bigAdd result: ${res}`);

console.log(hello.createBigIntI64());

console.log(hello.createBigInt());

hello.getCurrentDir((res) => {
  console.log(res);
});

hello.readFile((err, res) => {
  console.log(err, res);
});

try {
  hello.throwError();
} catch (e) {
  console.log(e);
}

console.log(hello.ClassWithFactory.withName('hello').name);

console.log(hello.CustomNumEnum.Eight);

console.log(hello.enumToI32(hello.CustomNumEnum.Six));

console.log(hello.returnNull());

console.log(hello.returnUndefined());

console.log(hello.mapOption(3));

console.log(hello.fibonacci(10));

console.log(hello.add(1, 2));

hello.asyncPlus100(
  new Promise((resolve) => {
    setTimeout(() => resolve(10), 100);
  })
);
