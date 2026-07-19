/* eslint-disable @typescript-eslint/no-explicit-any */

declare global {
  type AnyObject = Record<string, any>;

  type AnyArray = any[];

  type PromiseResult<T extends Promise> =
    ReturnType<T> extends Promise<infer R> ? R : never;

  type DeepPartial<T> = {
    [P in keyof T]?: T[P] extends AnyObject ? DeepPartial<T[P]> : T[P];
  };

  type PromiseReturnType<T> = Awaited<Promise<PromiseLike<T>>>;

  type ArrayElementType<T extends any[]> = T extends (infer U)[] ? U : never;

  type TupleToObject<
    T extends any[],
    K extends { [I in keyof T]: PropertyKey },
  > = {
    [I in keyof T as I extends keyof any[] ? never : K[I]]: T[I];
  };

  interface Box<T = any> {
    value: T;
  }

  type OmitNestedFields<T, R> = Omit<T, keyof R> & R;
}

export default global;
