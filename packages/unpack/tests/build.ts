
import { Compiler } from "../src/index.ts"
import path from 'path';
const registry = new FinalizationRegistry((value) => {
    console.log(`${value} gc collected`);
});
console.log('dir:',import.meta.dirname)
async function main() {
    const compiler = new Compiler({
        context: path.resolve(import.meta.dirname,'./fixtures'),
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
    registry.register(compiler, 'compiler');
    console.log('build1');
    await compiler.build()
    console.log('build2')
    await compiler.build();
    console.log('build3')
    
    console.log('build finished');
}
main().then(() => {
    if (global.gc) {
        global.gc();
    }
})
