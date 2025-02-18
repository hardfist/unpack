
const { JsCompiler } = require('..');
const binding = require('..');
const path = require('path');
const context = path.resolve(__dirname, './fixtures');
const registry = new FinalizationRegistry((value) => {
    console.log(`${value} gc collected`);
});
async function main() {
    const compiler = new JsCompiler(context, './src/index.mjs', [{
        onResolve(p) {
            // console.log('resolve:', p)
        },
        thisCompilation(compilation) {
            //console.log('thisCompilation:', compilation)
        }
    }]);
    await compiler.build()
    await compiler.build();
    registry.register(compiler, 'compiler');
    console.log('build finished');
}
main().then(() => {
    if (global.gc) {
        global.gc();
    }
})
