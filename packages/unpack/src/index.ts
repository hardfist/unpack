import type {JsPluginAdapter} from '@unpack-js/binding';
import { JsCompiler } from '@unpack-js/binding';

interface CompilerOptions {
    context: string,
    entry: string,
    plugins: Array<JsPluginAdapter>
}
export class Compiler {
    inner: JsCompiler;
    constructor(options: CompilerOptions ){
        this.inner = new JsCompiler(options.context, options.entry, options.plugins)
    }
    async build(){

    }
}
class Compilation {

}