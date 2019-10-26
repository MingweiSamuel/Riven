const util = require('util');
const fs = require('fs');
fs.readFileAsync = util.promisify(fs.readFile);
fs.writeFileAsync = util.promisify(fs.writeFile);
const req = require("request-promise-native");

process.chdir(__dirname);

const files = [
  [
    'http://raw.communitydragon.org/pbe/plugins/rcp-be-lol-game-data/global/default/v1/champion-summary.json',
    '.champion.json'
  ],
  [
    'http://www.mingweisamuel.com/riotapi-schema/openapi-3.0.0.json',
    '.spec.json'
  ],
  [
    'http://static.developer.riotgames.com/docs/lol/seasons.json',
    '.seasons.json'
  ],
  [
    'http://static.developer.riotgames.com/docs/lol/queues.json',
    '.queues.json'
  ],
  [
    'http://static.developer.riotgames.com/docs/lol/gameTypes.json',
    '.gameTypes.json'
  ],
  [
    'http://static.developer.riotgames.com/docs/lol/gameModes.json',
    '.gameModes.json'
  ]
]

const downloadFilesPromise = Promise.all(files.map(([url, file]) => req(url)
  .then(body => fs.writeFileAsync(file, body, "utf8"))));

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
    .map(file => fs.readFileAsync(file, "utf8")
      .then(input => {
        try {
          return doT.template(input)({});
        }
        catch (e) {
          console.error(`Error thrown while running "${file}":`, e);
        }
      })
      .then(output => fs.writeFileAsync("../src/" + file.slice(0, -suffix.length), output, "utf8"))
    )
  ))
  .catch(console.error);