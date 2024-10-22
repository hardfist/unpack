const binding = require('..');
const path = require('path');
const context = path.resolve(__dirname, './fixtures');
console.log('context:', context);
binding.build(context, './src/index.mjs', [{
    onResolve(p){
        console.log(p)
    }
}])