const binding = require('..');
const path = require('path');
const fs = require('fs');
const context = path.resolve(__dirname, '../../../benchmark/performance-compare-ng/apps/10000');
binding.build(context, './src/index.jsx', [{
    // onResolve(path) {
    //     return undefined;
    // },
    // onLoad(path){
    //     console.log('path:',path);
    //     const content = fs.readFile(path);
    //     return content;
    // }
}])