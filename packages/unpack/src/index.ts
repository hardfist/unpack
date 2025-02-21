import type {JsCompilation, JsPluginAdapter} from '@unpack-js/binding';
import { JsCompiler } from '@unpack-js/binding';

interface CompilerOptions {
    context: string,
    entry: string,
    plugins: Array<JsPluginAdapter>
}
class Compilation {
    inner: JsCompilation;
    compiler: Compiler;
    constructor(compilation: JsCompilation,compiler: Compiler){
        this.inner = compilation;
        this.compiler = compiler;
    }
}
export class Compiler {
    inner: JsCompiler;
    constructor(options: CompilerOptions ){
        this.inner = new JsCompiler(options.context, options.entry, options.plugins)
    }
    // inject Compilation 
    decoratePlugin(plugins: Array<JsPluginAdapter>){
        return plugins.map(plugin => {
            return {
                ...plugin,
                thisCompilation(compilation: any){
                    
                    // wrap compilation
                    plugin.thisCompilation(new Compilation(compilation,this));
                }
            }
        })
        
    }
    async build(){
        await this.inner.build();
    }
}