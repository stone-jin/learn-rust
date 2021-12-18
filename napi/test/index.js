const { sync, sleep } = require('../')


sleep(1000).then(() => {
    let result = sync(100);
    console.log(result);
})