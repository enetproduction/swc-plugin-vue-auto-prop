export type Prop<T> =
    | { (): T }
    | { new (...args: never[]): T & object }
    | { new (...args: string[]): Function }

export type PropType<T> = Prop<T> | Prop<T>[]

export interface PropOptions<T = any> {
    type?: PropType<T>
    required?: boolean
    default?: T | null | undefined | (() => T | null | undefined)
    validator?(value: unknown): boolean
}
export function Prop<T extends unknown>(options: PropOptions = {}): Function {
    return function decorator(target: T, propertyName: string) {
        // some code here
    };
}
