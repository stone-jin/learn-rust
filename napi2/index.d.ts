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
