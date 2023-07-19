export function deepMerge<T extends any[]>(...objs: T): T[number] {
  function getType(obj: unknown) {
    return Object.prototype.toString.call(obj).slice(8, -1).toLowerCase();
  }

  function mergeObj(clone: { [x: string]: unknown }, obj: { [s: string]: unknown } | ArrayLike<unknown>) {
    for (let [key, value] of Object.entries(obj)) {
      let type = getType(value);
      if (clone[key] !== undefined && getType(clone[key]) === type && ['array', 'object'].includes(type)) {
        clone[key] = deepMerge(clone[key], value);
      } else {
        clone[key] = structuredClone(value);
      }
    }
  }

  let clone = structuredClone(objs.shift());

  for (let obj of objs) {
    let type = getType(obj);

    if (getType(clone) !== type) {
      clone = structuredClone(obj);
      continue;
    }

    if (type === 'array') {
      clone = [...clone, ...structuredClone(obj)];
    } else if (type === 'object') {
      mergeObj(clone, obj);
    } else {
      clone = obj;
    }
  }

  return clone;
}
