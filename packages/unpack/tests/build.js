
import { Compiler } from '../src/index.ts'
import path from 'path';
const context = import.meta.resolve(import.meta.dirname, './fixtures');
const registry = new FinalizationRegistry((value) => {
    console.log(`${value} gc collected`);
});
async function main() {
    const compiler = new Compiler({
        context,
        entry: './src/index.mjs',
        plugins: [
            {
                onResolve(p) {
                    // console.log('resolve:', p)
                },
                thisCompilation(compilation) {
                    //console.log('thisCompilation:', compilation)
                }
            }
        ]
    });
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
