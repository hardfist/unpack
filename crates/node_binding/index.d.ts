/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export interface JsPluginAdapter {
  onResolve?: (arg: string) => any
  onLoad?: (arg: string) => any
}
export declare class JsCompilation {
  get compiler(): JsCompiler
}
export declare class JsCompiler {
  constructor(context: string, entry: string, plugins: Array<JsPluginAdapter>)
  build(): Promise<void>
}
