type RandomObject = { [x: string]: any };

function deepMerge<T extends RandomObject, X extends RandomObject>(target: T, source: X) {
  const result = { ...target, ...source };
  for (const key of Object.keys(result)) {
    (result[key] as RandomObject) =
      typeof target[key] == 'object' && typeof source[key] == 'object'
        ? deepMerge(target[key], source[key])
        : structuredClone(result[key]);
  }
  return result;
}
