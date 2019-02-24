export type Newtype = number;

export const isNewtype = (obj: any): obj is Newtype => {
  if (!(typeof obj === "number")) return false;
  return true;
};

export type Point = { X: number; Y: number; z: number };

export const isPoint = (obj: any): obj is Point => {
  if (obj == undefined) return false;
  if (obj.X === undefined) return false;
  {
    const val = obj.X;
    if (!(typeof val === "number")) return false;
  }
  if (obj.Y === undefined) return false;
  {
    const val = obj.Y;
    if (!(typeof val === "number")) return false;
  }
  if (obj.z === undefined) return false;
  {
    const val = obj.z;
    if (!(typeof val === "number")) return false;
  }
  return true;
};

export type Enum =
  | { V1: { Foo: boolean } }
  | { V2: { Bar: number; Baz: number } }
  | { V3: { Quux: string } };

export const isEnum = (obj: any): obj is Enum => {
  if (obj == undefined) return false;
  if (
    (() => {
      const v = obj.V1;
      if (v == undefined) return false;
      if (v.Foo === undefined) return false;
      {
        const val = v.Foo;
        if (!(typeof val === "boolean")) return false;
      }
      return true;
    })()
  )
    return true;
  if (
    (() => {
      const v = obj.V2;
      if (v == undefined) return false;
      if (v.Bar === undefined) return false;
      {
        const val = v.Bar;
        if (!(typeof val === "number")) return false;
      }
      if (v.Baz === undefined) return false;
      {
        const val = v.Baz;
        if (!(typeof val === "number")) return false;
      }
      return true;
    })()
  )
    return true;
  if (
    (() => {
      const v = obj.V3;
      if (v == undefined) return false;
      if (v.Quux === undefined) return false;
      {
        const val = v.Quux;
        if (!(typeof val === "string")) return false;
      }
      return true;
    })()
  )
    return true;
  return false;
};

export type Value<T> = { value: T };

export const isValue = <T>(obj: any, typename: string): obj is Value<T> => {
  if (obj == undefined) return false;
  if (obj.value === undefined) return false;
  {
    const val = obj.value;
    if (!isT<T>(val, typename)) return false;
  }
  return true;
};
// generic test
export const isT = <T>(val: any, typename: string): val is T => {
  return typeof val === typename;
};

export type DependsOnValue = { value: Value<number>[] };

export const isDependsOnValue = (obj: any): obj is DependsOnValue => {
  if (obj == undefined) return false;
  if (obj.value === undefined) return false;
  {
    const val = obj.value;
    if (!Array.isArray(val)) return false;
    for (let x of val) {
      if (!isValue<number>(x, "number")) return false;
    }
  }
  return true;
};

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

export const isFrontendMessage = (obj: any): obj is FrontendMessage => {
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

export const isBorrow = (obj: any, typename: string): obj is Borrow => {
  if (obj == undefined) return false;
  if (obj.raw === undefined) return false;
  {
    const val = obj.raw;
    if (!(typeof val === "string")) return false;
  }
  if (obj.cow === undefined) return false;
  {
    const val = obj.cow;
    if (!(typeof val === "string")) return false;
  }
  if (obj.map === undefined) return false;
  {
    const val = obj.map;
    if (val === null || !(typeof val === "object")) return false;
    for (let k in val) {
      let v = val[k];
      if (!(typeof v === "number")) return false;
    }
  }
  if (obj.array === undefined) return false;
  {
    const val = obj.array;
    if (!Array.isArray(val)) return false;
    for (let x of val) {
      if (!(typeof x === "string")) return false;
    }
  }
  return true;
};

export type IntMap = { intmap: { [key: number]: number } };

export const isIntMap = (obj: any): obj is IntMap => {
  if (obj == undefined) return false;
  if (obj.intmap === undefined) return false;
  {
    const val = obj.intmap;
    if (val === null || !(typeof val === "object")) return false;
    for (let k in val) {
      let v = val[k];
      if (+k === NaN) return false;
      if (!(typeof v === "number")) return false;
    }
  }
  return true;
};

export type MyBytes = { buffer: string };

export const isMyBytes = (obj: any): obj is MyBytes => {
  if (obj == undefined) return false;
  if (obj.buffer === undefined) return false;
  {
    const val = obj.buffer;
    if (!(typeof val === "string")) return false;
  }
  return true;
};

export type S =
  | { kind: "A" }
  | { kind: "E2"; fields: { key: number; a: number } }
  | { kind: "F"; fields: [number, string] };

export const isS = (obj: any): obj is S => {
  if (obj == undefined) return false;
  if (
    (() => {
      return obj.kind === "A";
    })()
  )
    return true;
  if (
    (() => {
      if (!(obj.kind === "E2")) return false;
      const v = obj.fields;
      if (v == undefined) return false;
      if (v.key === undefined) return false;
      {
        const val = v.key;
        if (!(typeof val === "number")) return false;
      }
      if (v.a === undefined) return false;
      {
        const val = v.a;
        if (!(typeof val === "number")) return false;
      }
      return true;
    })()
  )
    return true;
  if (
    (() => {
      if (!(obj.kind === "F")) return false;
      const v = obj.fields;
      if (!Array.isArray(v) || !(v.length === 2)) return false;
      if (v[0] === undefined) return false;
      {
        const val = v[0];
        if (!(typeof val === "number")) return false;
      }
      if (v[1] === undefined) return false;
      {
        const val = v[1];
        if (!(typeof val === "string")) return false;
      }
      return true;
    })()
  )
    return true;
  return false;
};

export type Address = { number: number; street: string; zip: number };

export const isAddress = (obj: any): obj is Address => {
  if (obj == undefined) return false;
  if (obj.number === undefined) return false;
  {
    const val = obj.number;
    if (!(typeof val === "number")) return false;
  }
  if (obj.street === undefined) return false;
  {
    const val = obj.street;
    if (!(typeof val === "string")) return false;
  }
  if (obj.zip === undefined) return false;
  {
    const val = obj.zip;
    if (!(typeof val === "number")) return false;
  }
  return true;
};

export type Record = { name: string; address: Address; year: number | null };

export const isRecord = (obj: any): obj is Record => {
  if (obj == undefined) return false;
  if (obj.name === undefined) return false;
  {
    const val = obj.name;
    if (!(typeof val === "string")) return false;
  }
  if (obj.address === undefined) return false;
  {
    const val = obj.address;
    if (!isAddress(val)) return false;
  }
  if (obj.year === undefined) return false;
  {
    const val = obj.year;
    if (!(val === null)) {
      if (!(typeof val === "number")) return false;
    }
  }
  return true;
};

export type Search = { results: { Ok: Record[] } | { Err: string } };

export const isSearch = (obj: any): obj is Search => {
  if (obj == undefined) return false;
  if (obj.results === undefined) return false;
  {
    const val = obj.results;
    {
      if (val === null) return false;
      if (
        (v => {
          if (v == undefined) return false;
          if (!Array.isArray(v)) return false;
          for (let x of v) {
            if (!isRecord(x)) return false;
            break;
          }
          return true;
        })(val.Ok) ||
        (v => {
          if (v == undefined) return false;
          if (!(typeof v === "string")) return false;
          return true;
        })(val.Err)
      )
        return true;
      return false;
    }
  }
  return true;
};

export enum TyEnum {
  Red = "Red",
  Green = "Green",
  Blue = "Blue"
}

export const isTyEnum = (obj: any): obj is TyEnum => {
  if (!(obj === "Red" || obj === "Green" || obj === "Blue")) return false;
  return true;
};

export type Value2<T> = { value: T };

export const isValue2 = <T>(obj: any, typename: string): obj is Value2<T> => {
  if (obj == undefined) return false;
  if (obj.value === undefined) return false;
  {
    const val = obj.value;
    {
      if (!(typeof val === "object")) return false;
      return true;
    }
  }
  return true;
};

export type DependsOnValue2 = { value: Value2<number> };

export const isDependsOnValue2 = (obj: any): obj is DependsOnValue2 => {
  if (obj == undefined) return false;
  if (obj.value === undefined) return false;
  {
    const val = obj.value;
    if (!isValue2<number>(val, "number")) return false;
  }
  return true;
};

export type Chrono = {
  duration: { secs: number; nanos: number };
  systime: { secs_since_epoch: number; nanos_since_epoch: number };
  dt: string;
  path: string;
};

export const isChrono = (obj: any): obj is Chrono => {
  if (obj == undefined) return false;
  if (obj.duration === undefined) return false;
  {
    const val = obj.duration;
    {
      if (val === null) return false;
      if (!(typeof val.secs === "number")) return false;
      if (!(typeof val.nanos === "number")) return false;
    }
  }
  if (obj.systime === undefined) return false;
  {
    const val = obj.systime;
    {
      if (val === null) return false;
      if (!(typeof val.secs_since_epoch === "number")) return false;
      if (!(typeof val.nanos_since_epoch === "number")) return false;
    }
  }
  if (obj.dt === undefined) return false;
  {
    const val = obj.dt;
    if (!(typeof val === "string")) return false;
  }
  if (obj.path === undefined) return false;
  {
    const val = obj.path;
    if (!(typeof val === "string")) return false;
  }
  return true;
};

export type Either = { either: { Left: Address } | { Right: string } };

export const isEither = (obj: any): obj is Either => {
  if (obj == undefined) return false;
  if (obj.either === undefined) return false;
  {
    const val = obj.either;
    {
      if (val === null) return false;
      if (
        (v => {
          if (v == undefined) return false;
          if (!isAddress(v)) return false;
          return true;
        })(val.Left) ||
        (v => {
          if (v == undefined) return false;
          if (!(typeof v === "string")) return false;
          return true;
        })(val.Right)
      )
        return true;
      return false;
    }
  }
  return true;
};
