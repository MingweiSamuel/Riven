const fs = require('fs/promises');
const fsSync = require('fs');
const path = require('path');
const { parseArgs } = require('node:util');

const { values: argv } = parseArgs({
  options: {
    spec: {
      type: 'string'
    }
  }
});

const defaultSchemaSource = 'http://www.mingweisamuel.com/riotapi-schema';
let schemaSource = argv.spec || defaultSchemaSource;
const isRemoteSource = /^https?:\/\//.test(schemaSource);

if (!isRemoteSource) {
  schemaSource = path.resolve(process.cwd(), schemaSource);
  if (!fsSync.existsSync(schemaSource) || !fsSync.statSync(schemaSource).isDirectory()) {
    throw new Error(
      `--spec must point to a schema source directory (typically riotapi-schema/out). Received: ${schemaSource}`
    );
  }
}

const trimTrailingSlash = s => s.replace(/\/+$/, '');
const resolveFromSource = relPath => {
  if (isRemoteSource)
    return `${trimTrailingSlash(schemaSource)}/${relPath}`;
  return path.join(schemaSource, ...relPath.split('/'));
};

const files = [
  [
    'http://raw.communitydragon.org/pbe/plugins/rcp-be-lol-game-data/global/default/v1/champion-summary.json',
    '.champion.json'
  ],
  [
    resolveFromSource('openapi-3.0.0.json'),
    '.spec.json'
  ],
  [
    resolveFromSource('enums/seasons.json'),
    '.seasons.json'
  ],
  [
    resolveFromSource('enums/queues.json'),
    '.queues.json'
  ],
  [
    resolveFromSource('enums/queueTypes.json'),
    '.queueTypes.json'
  ],
  [
    resolveFromSource('enums/gameTypes.json'),
    '.gameTypes.json'
  ],
  [
    resolveFromSource('enums/gameModes.json'),
    '.gameModes.json'
  ],
  [
    resolveFromSource('enums/maps.json'),
    '.maps.json'
  ],
  [
    resolveFromSource('routesTable.json'),
    '.routesTable.json'
  ],
];

if (argv.spec) console.log(`Using custom schema source: ${schemaSource}`);

const downloadFilesPromise = Promise.all(files.map(async ([url, file]) => {
  let body;
  if (url.startsWith('http')) {
    const req = await fetch(url);
    body = await req.text();
  }
  else {
    body = await fs.readFile(url, "utf8");
  }
  await fs.writeFile(path.join(__dirname, file), body, "utf8");
}));

const doT = require('dot');
const glob = require('glob-promise');

const log = a => { console.log(a); return a; };
const suffix = '.dt';

doT.templateSettings = {
  evaluate: /\r?\n?\{\{([\s\S]+?)\}\}/g,
  interpolate: /\r?\n?\{\{=([\s\S]+?)\}\}/g,
  encode: /\r?\n?\{\{!([\s\S]+?)\}\}/g,
  use: /\r?\n?\{\{#([\s\S]+?)\}\}/g,
  define: /\r?\n?\{\{##\s*([\w\.$]+)\s*(\:|=)([\s\S]+?)#\}\}/g,
  conditional: /\r?\n?\{\{\?(\?)?\s*([\s\S]*?)\s*\}\}/g,
  iterate: /\r?\n?\{\{~\s*(?:\}\}|([\s\S]+?)\s*\:\s*([\w$]+)\s*(?:\:\s*([\w$]+))?\s*\}\})/g,
  varname: 'it',
  strip: false,
  append: false,
  selfcontained: false
};

global.require = require;

downloadFilesPromise.then(() => glob.promise(path.join(__dirname, "**/*" + suffix), { ignore: ["**/node_modules/**"] }))
  .then(files => Promise.all(files
    .map(log)
    .map(file => fs.readFile(file, "utf8")
      .then(input => {
        try {
          return doT.template(input)({});
        }
        catch (e) {
          console.error(`Error thrown while running "${file}":`, e);
          throw e;
        }
      })
      .then(output => fs.writeFile(file.replace(/\bsrcgen\b/, "src").slice(0, -suffix.length), output, "utf8"))
    )
  ))
  .catch(console.error);
