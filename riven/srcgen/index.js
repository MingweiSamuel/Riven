const fs = require('fs/promises');
const { parseArgs } = require('node:util');

process.chdir(__dirname);

const { values: argv } = parseArgs({
  options: {
    spec: {
      type: 'string'
    }
  }
});

const files = [
  [
    'http://raw.communitydragon.org/pbe/plugins/rcp-be-lol-game-data/global/default/v1/champion-summary.json',
    '.champion.json'
  ],
  [
    argv.spec || 'http://www.mingweisamuel.com/riotapi-schema/openapi-3.0.0.json',
    '.spec.json'
  ],
  [
    'http://www.mingweisamuel.com/riotapi-schema/enums/seasons.json',
    '.seasons.json'
  ],
  [
    'http://www.mingweisamuel.com/riotapi-schema/enums/queues.json',
    '.queues.json'
  ],
  [
    'http://www.mingweisamuel.com/riotapi-schema/enums/queueTypes.json',
    '.queueTypes.json'
  ],
  [
    'http://www.mingweisamuel.com/riotapi-schema/enums/gameTypes.json',
    '.gameTypes.json'
  ],
  [
    'http://www.mingweisamuel.com/riotapi-schema/enums/gameModes.json',
    '.gameModes.json'
  ],
  [
    'http://www.mingweisamuel.com/riotapi-schema/enums/maps.json',
    '.maps.json'
  ],
  [
    'http://www.mingweisamuel.com/riotapi-schema/routesTable.json',
    '.routesTable.json'
  ],
];

if (argv.spec) console.log(`Using custom spec file: ${argv.spec}`);

const downloadFilesPromise = Promise.all(files.map(async ([url, file]) => {
  let body;
  if (url.startsWith('http')) {
    const req = await fetch(url);
    body = await req.text();
  }
  else {
    body = await fs.readFile(url, "utf8");
  }
  await fs.writeFile(file, body, "utf8");
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

downloadFilesPromise.then(() => glob.promise("**/*" + suffix, { ignore: ["**/node_modules/**"] }))
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
      .then(output => fs.writeFile("../src/" + file.slice(0, -suffix.length), output, "utf8"))
    )
  ))
  .catch(console.error);