const binding = require('..');
const path = require('path');
const fs = require('fs/promises');
const context = path.resolve(__dirname, '../../../benchmark/performance-compare-ng/apps/10000');
binding.build(context, './src/index.jsx', [{
    async onLoad(path){
        return fs.readFile(path);
    }
}])