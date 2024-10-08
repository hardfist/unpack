import test from 'node:test'
import path from 'node:path';
import binding from '../index.js'
test('basic', (t) => {
    const context = path.resolve(import.meta.dirname, './fixtures');
    console.log('context:',context);
    binding.build(context, './src/index.mjs',function hook(err, x,cb){
        console.log('sync');
        return `${x}-${x}`
    })
    binding.build(context, './src/index.mjs',async function hook(err, x,cb){
        console.log('async');
        return `${x}-${x}-${x}`
    })
})