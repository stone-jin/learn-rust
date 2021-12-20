/* eslint-disable */

export class ExternalObject<T> {
  readonly '': {
    readonly '': unique symbol
    [K: symbol]: T
  }
}
export function sum(a: number, b: number): number
export function getStr(): Array<string>
export function getNums(): Array<number>
export function sumNums(nums: Array<number>): number
export function readFileAsync(path: string): Promise<Buffer>
export function asyncMultiTwo(num: number): Promise<number>
export function bigAdd(a: BigInt, b: BigInt): BigInt
export function createBigIntI64(): BigInt
export function createBigInt(): BigInt
export function getCurrentDir(callback: (arg0: string) => void): void
export function readFile(callback: (arg0: Error | undefined, arg1?: string | undefined | null) => void): void
export function throwError(): void
export const enum Kind {
  Dog = 0,
  Cat = 1,
  Duck = 2
}
export const enum CustomNumEnum {
  One = 1,
  Two = 2,
  Three = 3,
  Four = 4,
  Six = 6,
  Eight = 8,
  Nine = 9,
  Ten = 10
}
export function enumToI32(e: CustomNumEnum): number
export function mapOption(val?: number | undefined | null): number | undefined | null
export function returnNull(): Null
export function returnUndefined(): Undefined
export function add(a: number, b: number): number
export function fibonacci(n: number): number
export function asyncPlus100(p: Promise<number>): Promise<number>
export class ClassWithFactory {
  name: string
  static withName(name: string): ClassWithFactory
  setName(name: string): this
}
export class Animal {
  readonly kind: Kind
  constructor(kind: Kind, name: string)
  static withKind(kind: Kind): Animal
  get name(): string
  set name(name: string)
  whoami(): string
  static getDogKind(): Kind
}
export class Blake2BHasher {
  static withKey(key: Blake2bKey): Blake2BHasher
}
export class Blake2BKey { }
export class Context {
  maybeNeed?: boolean | undefined | null
  constructor()
  static withData(data: string): Context
  method(): string
}
export class AnimalWithDefaultConstructor {
  name: string
  kind: number
  constructor(name: string, kind: number)
}
