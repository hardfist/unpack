const binding = require('..');
const path = require('path');
const context = path.resolve(__dirname, '../../../benchmark/performance-compare-ng/apps/10000');
binding.build(context, './src/index.jsx', [{
    // onResolve(err, path) {
    //     return undefined;
    // }
}])