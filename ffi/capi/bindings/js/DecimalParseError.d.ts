// generated by diplomat-tool
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";



/**
 * Additional information: [1](https://docs.rs/fixed_decimal/0.7.0/fixed_decimal/enum.ParseError.html)
 */
export class DecimalParseError {

    /** @internal */
    static fromValue(value: DecimalParseError | string): DecimalParseError;

    get value(): string;

    /** @internal */
    get ffiValue(): number;

    static Unknown : DecimalParseError;
    static Limit : DecimalParseError;
    static Syntax : DecimalParseError;


    constructor(value: DecimalParseError | string );
}