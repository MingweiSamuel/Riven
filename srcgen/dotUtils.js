// flatMap: https://gist.github.com/samgiles/762ee337dff48623e729
// [B](f: (A) ⇒ [B]): [B]  ; Although the types in the arrays aren't strict (:
Array.prototype.flatMap = function(lambda) {
  return Array.prototype.concat.apply([], this.map(lambda));
};
Array.prototype.groupBy = function(lambda) {
  return Object.entries(this.reduce((agg, x) => {
    const k = lambda(x);
    (agg[k] = agg[k] || []).push(x);
    return agg;
  }, {}));
}

function capitalize(input) {
  return input[0].toUpperCase() + input.slice(1);
}

function decapitalize(input) {
  return input[0].toLowerCase() + input.slice(1);
}

function normalizeEndpointName(name) {
  return name.split('-')
    .join('_');
}

function normalizeSchemaName(name) {
  return name.replace(/DTO/ig, '');
}

function normalizeArgName(name) {
  var tokens = name.split('_');
  var argName = decapitalize(tokens.map(capitalize).join(''));
  return 'base' === argName ? 'Base' : argName;
}

function normalizePropName(propName, schemaName, value) {
  var tokens = propName.split('_');
  var name = tokens.map(capitalize).join('');
  if (name === schemaName)
    name += stringifyType(value);
  return name;
}

function stringifyType(prop, endpoint = null, nullable = false) {
  if (prop.anyOf) {
    prop = prop.anyOf[0];
  }

  let refType = prop['$ref'];
  if (refType) {
    return (!endpoint ? '' : endpoint + '.') +
      normalizeSchemaName(refType.slice(refType.indexOf('.') + 1));
  }
  var qm = nullable ? '?' : '';
  switch (prop.type) {
    case 'boolean': return 'bool' + qm;
    case 'integer': return ('int32' === prop.format ? 'int' : 'long') + qm;
    case 'number': return prop.format + qm;
    case 'array': return stringifyType(prop.items, endpoint) + '[]' + qm;
    case 'object':
      return 'IDictionary<' + stringifyType(prop['x-key'], endpoint) + ', ' +
        stringifyType(prop.additionalProperties, endpoint) + '>' + qm;
    default: return prop.type + qm;
  }
}

function formatJsonProperty(name) {
  return `[JsonProperty(\"${name}\")]`;
}

function formatQueryParamStringify(prop) {
    switch (prop.type) {
        case 'boolean': return '.ToString().ToLowerInvariant()';
        case 'string': return '';
        default: return '.ToString()';
    }
}

function formatAddQueryParam(param) {
    let k = `nameof(${param.name})`;
    let nc = param.required ? '' : `if (null != ${param.name}) `;
    let prop = param.schema;
    switch (prop.type) {
        case 'array': return `${nc}queryParams.AddRange(${param.name}.Select(`
            + `w => new KeyValuePair<string, string>(${k}, w${formatQueryParamStringify(prop.items)})))`;
        case 'object': throw 'unsupported';
        default:
            let vnc = param.required ? '' : '.Value';
            return `${nc}queryParams.Add(new KeyValuePair<string, string>(${k}, `
            + `${param.name}${vnc}${formatQueryParamStringify(prop.type)}))`;
    }
}

module.exports = {
  capitalize,
  decapitalize,
  normalizeEndpointName,
  normalizeSchemaName,
  normalizeArgName,
  normalizePropName,
  stringifyType,
  formatJsonProperty,
  formatAddQueryParam
};