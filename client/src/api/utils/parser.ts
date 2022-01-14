export type Unparse<V> = V extends Date ? string : V

export type Unparsed<T extends object> = {
  [K in keyof T]: Unparse<T[K]>
}

export type UnparsedKeys<T extends object> = {
  [K in keyof T]: T[K] extends Date ? string : never
}[keyof T]

export function parse<T extends object>(object: Unparsed<T>, keys: UnparsedKeys<T>[]): T {
  for (const key of keys) {
    // @ts-expect-error
    if (object[key] !== null && object[key] !== undefined) {
      // @ts-expect-error
      object[key] = new Date(object[key])
    }
  }
  // @ts-expect-error
  return object
}
