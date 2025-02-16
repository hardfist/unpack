const { JsCompiler } = require('..');
const binding = require('..');
const path = require('path');
const context = path.resolve(__dirname, './fixtures');
console.log('context:', context);
async function main() {
    const compiler = new JsCompiler(context, './src/index.mjs', [{
        onResolve(p) {
            console.log('resolve:', p)
        }
    }]);
    await compiler.build()
    console.log('build finished');
}
main();

// setInterval(() => {

// }, 0).unref();