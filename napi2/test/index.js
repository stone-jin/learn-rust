var hello = require('../')
const path = require('path')

console.log(hello)

console.log(hello.sum(1, 2))

console.log(hello.getNums());

console.log(hello.getStr())

console.log(hello.sumNums([1, 2]))

hello.readFileAsync(path.join(__dirname, './index.js')).then(res => {
    // console.log(res.toString())
})

hello.asyncMultiTwo(2).then(res => {
    // console.log(res)
})

let res = hello.bigAdd(BigInt(1000), BigInt(1))
console.log(`bigAdd result: ${res}`)

console.log(hello.createBigIntI64())

console.log(hello.createBigInt())