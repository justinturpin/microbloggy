import resolve from '@rollup/plugin-node-resolve';
import replace from '@rollup/plugin-replace';
import commonjs from '@rollup/plugin-commonjs';
import alias from '@rollup/plugin-alias';
import vue from 'rollup-plugin-vue';
import css from 'rollup-plugin-css-only'
import { terser } from 'rollup-plugin-terser';

let nodeEnv = process.env.ROLLUP_WATCH ? 'development' : 'production';

export default [
  {
    input: 'js/index.js',
    output: [
      {
        file: "static/bundle.js",
        format: 'esm',
        sourcemap: true,
      }
    ],
    plugins: [
      replace({
        'process.env.NODE_ENV': JSON.stringify( nodeEnv ),
        '__VUE_PROD_DEVTOOLS__': JSON.stringify(true),
        '__VUE_OPTIONS_API__': JSON.stringify(true),
      }),
      alias({
        entries: [
          { find: 'vue', replacement: 'vue/dist/vue.esm-bundler.js' }
        ]
      }),
      css({
        output: "bundle.css"
      }),
      vue(),
      resolve({
        browser: true,
      }),
      commonjs(),
      // terser()
    ]
  }
]
