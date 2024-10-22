const binding = require('..');
const path = require('path');
const fs = require('fs');
const context = path.resolve(__dirname, '../../../benchmark/performance-compare-ng/apps/10000');
binding.build(context, './src/index.jsx', [{
    onLoad(path){
        const content = fs.readFileSync(path);
        return content;
    }
}])