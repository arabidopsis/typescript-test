export type Newtype = number;

export type Point = { X: number; Y: number; z: number };

export type Enum =
  | { V1: { Foo: boolean } }
  | { V2: { Bar: number; Baz: number } }
  | { V3: { Quux: string } };

export type Value<T> = { value: T };

export type DependsOnValue = { value: Value<number>[] };

// This is some API Event.
export type FrontendMessage =
  | { tag: "Init"; fields: { id: string } }
  | {
      tag: "ButtonState";
      fields: { selected: string[]; time: number; other: string | null };
    }
  | {
      tag: "Render";
      fields: {
        html: string;
        time: number;
        other_result: { Ok: string } | { Err: number };
      };
    }
  | { tag: "Stuff"; fields: { borrow: number[] } };

export const isa_FrontendMessage = (obj: any): obj is FrontendMessage => {
  if (obj == undefined) return false;
  if (
    (() => {
      if (!(obj.tag === "Init")) return false;
      const v = obj.fields;
      if (v == undefined) return false;
      if (v.id === undefined) return false;
      {
        const val = v.id;
        if (!(typeof val === "string")) return false;
      }
      return true;
    })()
  )
    return true;
  if (
    (() => {
      if (!(obj.tag === "ButtonState")) return false;
      const v = obj.fields;
      if (v == undefined) return false;
      if (v.selected === undefined) return false;
      {
        const val = v.selected;
        if (!Array.isArray(val)) return false;
        for (let x of val) {
          if (!(typeof x === "string")) return false;
        }
      }
      if (v.time === undefined) return false;
      {
        const val = v.time;
        if (!(typeof val === "number")) return false;
      }
      if (v.other === undefined) return false;
      {
        const val = v.other;
        if (!(val === null)) {
          if (!(typeof val === "string")) return false;
        }
      }
      return true;
    })()
  )
    return true;
  if (
    (() => {
      if (!(obj.tag === "Render")) return false;
      const v = obj.fields;
      if (v == undefined) return false;
      if (v.html === undefined) return false;
      {
        const val = v.html;
        if (!(typeof val === "string")) return false;
      }
      if (v.time === undefined) return false;
      {
        const val = v.time;
        if (!(typeof val === "number")) return false;
      }
      if (v.other_result === undefined) return false;
      {
        const val = v.other_result;
        {
          if (val === null) return false;
          if (
            (v => {
              if (v == undefined) return false;
              if (!(typeof v === "string")) return false;
              return true;
            })(val.Ok) ||
            (v => {
              if (v == undefined) return false;
              if (!(typeof v === "number")) return false;
              return true;
            })(val.Err)
          )
            return true;
          return false;
        }
      }
      return true;
    })()
  )
    return true;
  if (
    (() => {
      if (!(obj.tag === "Stuff")) return false;
      const v = obj.fields;
      if (v == undefined) return false;
      if (v.borrow === undefined) return false;
      {
        const val = v.borrow;
        if (!Array.isArray(val)) return false;
        for (let x of val) {
          if (!(typeof x === "number")) return false;
        }
      }
      return true;
    })()
  )
    return true;
  return false;
};

export type Borrow = {
  raw: string;
  cow: string;
  map: { [key: string]: number };
  array: string[];
};

export type MyBytes = { buffer: string };

export type S =
  | { kind: "A" }
  | { kind: "E2"; fields: { key: number; a: number } }
  | { kind: "F"; fields: [number, string] };

export type Address = { number: number; street: string; zip: number };

export type Record = { name: string; address: Address; year: number | null };

export type Search = { results: { Ok: Record[] } | { Err: string } };
