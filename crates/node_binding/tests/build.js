const { JsCompiler } = require('..');
const binding = require('..');
const path = require('path');
const context = path.resolve(__dirname, './fixtures');
console.log('context:', context);
const compiler = new JsCompiler(context, './src/index.mjs', [{
    onResolve(p){
        console.log(p)
    }
}])
compiler.build()