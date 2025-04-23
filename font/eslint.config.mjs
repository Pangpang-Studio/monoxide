import js from '@eslint/js'
import jsdoc from 'eslint-plugin-jsdoc'
import simpleImportSort from 'eslint-plugin-simple-import-sort'

const config = [
  js.configs.recommended,

  {
    plugins: {
      'simple-import-sort': simpleImportSort,
    },
    rules: {
      'simple-import-sort/imports': 'warn',
      'simple-import-sort/exports': 'warn',
    },
  },

  jsdoc.configs['flat/recommended'],
  {
    files: ['**/*.js'],
    plugins: {
      jsdoc,
    },
    rules: {
      'jsdoc/tag-lines': 0,
      'jsdoc/require-param-description': 0,
      'jsdoc/require-property-description': 0,
      'jsdoc/require-returns-description': 0,
    },
  },
]

export default config
