

<a id='definition__r7rs__current-jiffy'></a>

# `current-jiffy` -- `r7rs` Definitions


#### Kind

`procedure`;


#### Procedure signature

Procedure variants:
 * `(() |->| (|timestamp-jiffy|))`
   * inputs: none;
   * output: a value of type [`timestamp-jiffy`](../../r7rs/types/timestamp-jiffy.md#type__r7rs__timestamp-jiffy);


#### Referenced types

[`timestamp-jiffy`](../../r7rs/types/timestamp-jiffy.md#type__r7rs__timestamp-jiffy);


#### Description

> ````
> (current-jiffy)
> ````
> 
> 
> Returns the number of __jiffies__ as an exact integer that have elapsed since an arbitrary,
> implementation-defined epoch. A jiffy is an implementation-defined
> fraction of a second which is defined by the return value of the
> `jiffies-per-second` procedure. The starting epoch is guaranteed to be
> constant during a run of the program, but may vary between runs.
> 
> **Rationale**:  Jiffies are allowed to be implementation-dependent so that
> `current-jiffy` can execute with minimum overhead. It
> should be very likely that a compactly represented integer will suffice
> as the returned value.  Any particular jiffy size will be inappropriate
> for some implementations: a microsecond is too long for a very fast
> machine, while a much smaller unit would force many implementations to
> return integers which have to be allocated for most calls, rendering
> `current-jiffy` less useful for accurate timing measurements.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:time`](../../r7rs/categories/r7rs_3a_time.md#category__r7rs__r7rs_3a_time);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----
