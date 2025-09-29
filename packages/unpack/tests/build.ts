
import { Compiler } from "../src/index.ts"
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const registry = new FinalizationRegistry<string>((value) => {
    console.log(`${value} gc collected`);
});
console.log('dir:', __dirname)
async function main() {
    const compiler = new Compiler({
        context: path.resolve(__dirname,'./fixtures'),
        entry: './src/index.mjs',
        plugins: [
            {
                onResolve(p) {
                    // console.log('resolve:', p)
                },
                thisCompilation(compilation) {
                    // registry.register(compilation, 'compilation');
                    //console.log('thisCompilation:', compilation)
                   setTimeout(() => {
                     console.log('after 1s ---------\n\n\n');
                     console.log('compilation:', compilation)
                   },1000);
                }
            }
        ]
    });
    registry.register(compiler, 'compiler');
    console.log('build1');
    await compiler.build()
    console.log('build2')
    await compiler.build();
    
    console.log('build finished');
}
main().then(() => {
    const g: any = globalThis as any;
    if (g.gc) {
        g.gc();
    }
})
