const binding = require('..');
const path = require('path');
const fs = require('fs/promises');
const context = path.resolve(__dirname, '../../../benchmark/performance-compare-ng/apps/10000-big');
binding.build(context, './src/index.jsx', [])