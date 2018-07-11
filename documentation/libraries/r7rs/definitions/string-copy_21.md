

<a id='definition__r7rs__string-copy_21'></a>

# `string-copy!` -- `r7rs` Definitions


#### Kind

`mutator!`;


#### Procedure signature

Procedure variants:
 * `(((|source| . |string|) (|source-start| . |range-start|) (|destination| . |string|)) |->| (|void|))`
   * inputs:
     * `source` of type [`string`](../../r7rs/types/string.md#type__r7rs__string);
     * `source-start` of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
     * `destination` of type [`string`](../../r7rs/types/string.md#type__r7rs__string);
   * output: a value of type [`void`](../../r7rs/types/void.md#type__r7rs__void);
 * `(((|source| . |string|) (|source-start| . |range-start|) (|destination| . |string|) (|destination-start| . |range-start|)) |->| (|void|))`
   * inputs:
     * `source` of type [`string`](../../r7rs/types/string.md#type__r7rs__string);
     * `source-start` of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
     * `destination` of type [`string`](../../r7rs/types/string.md#type__r7rs__string);
     * `destination-start` of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
   * output: a value of type [`void`](../../r7rs/types/void.md#type__r7rs__void);
 * `(((|source| . |string|) (|source-start| . |range-start|) (|destination| . |string|) (|destination-start| . |range-start|) (|destination-end| . |range-end|)) |->| (|void|))`
   * inputs:
     * `source` of type [`string`](../../r7rs/types/string.md#type__r7rs__string);
     * `source-start` of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
     * `destination` of type [`string`](../../r7rs/types/string.md#type__r7rs__string);
     * `destination-start` of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
     * `destination-end` of type [`range-end`](../../r7rs/types/range-end.md#type__r7rs__range-end);
   * output: a value of type [`void`](../../r7rs/types/void.md#type__r7rs__void);


#### Referenced types

[`string`](../../r7rs/types/string.md#type__r7rs__string);
[`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
[`void`](../../r7rs/types/void.md#type__r7rs__void);
[`range-end`](../../r7rs/types/range-end.md#type__r7rs__range-end);


#### Description

> ````
> (string-copy! to at from)
> (string-copy! to at from start)
> (string-copy! to at from start end)
> ````
> 
> 
> **Domain**:  It is an error if `at` is less than zero or greater than the length of `to`.
> It is also an error if `(- (string-length to) at)`
> is less than `(- end start)`.
> 
> Copies the characters of string `from` between `start` and `end`
> to string `to`, starting at `at`.  The order in which characters are
> copied is unspecified, except that if the source and destination overlap,
> copying takes place as if the source is first copied into a temporary
> string and then into the destination.  This can be achieved without
> allocating storage by making sure to copy in the correct direction in
> such circumstances.
> 
> ````
> (define a "12345")
> (define b (string-copy "abcde"))
> (string-copy! b 1 a 0 2)
> b  ===>  "a12de"
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:strings`](../../r7rs/categories/vs_3a_strings.md#category__r7rs__vs_3a_strings);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----
