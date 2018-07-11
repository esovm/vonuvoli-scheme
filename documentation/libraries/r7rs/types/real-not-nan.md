

<a id='type__r7rs__real-not-nan'></a>

# `real-not-nan` -- `r7rs` Types


#### Sub-types tree

* **[`real-not-inf-not-nan`](../../r7rs/types/real-not-inf-not-nan.md#type__r7rs__real-not-inf-not-nan)**:
  * **[`rational`](../../r7rs/types/rational.md#type__r7rs__rational)**:
    * **[`integer`](../../r7rs/types/integer.md#type__r7rs__integer)**:
      * ...
    * **[`exact-rational`](../../r7rs/types/exact-rational.md#type__r7rs__exact-rational)**:
      * ...
    * **[`inexact-rational`](../../r7rs/types/inexact-rational.md#type__r7rs__inexact-rational)**:
      * ...
    * **[`rational-not-zero`](../../r7rs/types/rational-not-zero.md#type__r7rs__rational-not-zero)**:
      * ...
    * **[`rational-positive-or-zero`](../../r7rs/types/rational-positive-or-zero.md#type__r7rs__rational-positive-or-zero)**:
      * ...
    * **[`rational-negative-or-zero`](../../r7rs/types/rational-negative-or-zero.md#type__r7rs__rational-negative-or-zero)**:
      * ...
  * **[`exact-real`](../../r7rs/types/exact-real.md#type__r7rs__exact-real)**:
    * [`exact-rational`](../../r7rs/types/exact-rational.md#type__r7rs__exact-rational):
      * ...
  * **[`real-zero`](../../r7rs/types/real-zero.md#type__r7rs__real-zero)**:
    * **[`rational-zero`](../../r7rs/types/rational-zero.md#type__r7rs__rational-zero)**:
      * ...
  * **[`inexact-real-not-inf-not-nan`](../../r7rs/types/inexact-real-not-inf-not-nan.md#type__r7rs__inexact-real-not-inf-not-nan)**:
    * [`inexact-rational`](../../r7rs/types/inexact-rational.md#type__r7rs__inexact-rational):
      * ...
* **[`real-not-zero-not-nan`](../../r7rs/types/real-not-zero-not-nan.md#type__r7rs__real-not-zero-not-nan)**:
  * **[`real-positive`](../../r7rs/types/real-positive.md#type__r7rs__real-positive)**:
    * **[`real-positive-not-inf`](../../r7rs/types/real-positive-not-inf.md#type__r7rs__real-positive-not-inf)**:
      * ...
  * **[`real-negative`](../../r7rs/types/real-negative.md#type__r7rs__real-negative)**:
    * **[`real-negative-not-inf`](../../r7rs/types/real-negative-not-inf.md#type__r7rs__real-negative-not-inf)**:
      * ...
* **[`real-positive-or-zero`](../../r7rs/types/real-positive-or-zero.md#type__r7rs__real-positive-or-zero)**:
  * [`real-positive`](../../r7rs/types/real-positive.md#type__r7rs__real-positive):
    * [`real-positive-not-inf`](../../r7rs/types/real-positive-not-inf.md#type__r7rs__real-positive-not-inf):
      * ...
  * **[`real-positive-or-zero-not-inf`](../../r7rs/types/real-positive-or-zero-not-inf.md#type__r7rs__real-positive-or-zero-not-inf)**:
    * [`real-zero`](../../r7rs/types/real-zero.md#type__r7rs__real-zero):
      * ...
    * [`real-positive-not-inf`](../../r7rs/types/real-positive-not-inf.md#type__r7rs__real-positive-not-inf):
      * ...
    * [`rational-positive-or-zero`](../../r7rs/types/rational-positive-or-zero.md#type__r7rs__rational-positive-or-zero):
      * ...
* **[`real-negative-or-zero`](../../r7rs/types/real-negative-or-zero.md#type__r7rs__real-negative-or-zero)**:
  * [`real-negative`](../../r7rs/types/real-negative.md#type__r7rs__real-negative):
    * [`real-negative-not-inf`](../../r7rs/types/real-negative-not-inf.md#type__r7rs__real-negative-not-inf):
      * ...
  * **[`real-negative-or-zero-not-inf`](../../r7rs/types/real-negative-or-zero-not-inf.md#type__r7rs__real-negative-or-zero-not-inf)**:
    * [`real-zero`](../../r7rs/types/real-zero.md#type__r7rs__real-zero):
      * ...
    * [`real-negative-not-inf`](../../r7rs/types/real-negative-not-inf.md#type__r7rs__real-negative-not-inf):
      * ...
    * [`rational-negative-or-zero`](../../r7rs/types/rational-negative-or-zero.md#type__r7rs__rational-negative-or-zero):
      * ...
* **[`inexact-real-not-nan`](../../r7rs/types/inexact-real-not-nan.md#type__r7rs__inexact-real-not-nan)**:
  * **[`real-inf`](../../r7rs/types/real-inf.md#type__r7rs__real-inf)**;
  * [`inexact-real-not-inf-not-nan`](../../r7rs/types/inexact-real-not-inf-not-nan.md#type__r7rs__inexact-real-not-inf-not-nan):
    * [`inexact-rational`](../../r7rs/types/inexact-rational.md#type__r7rs__inexact-rational):
      * ...


#### Super-type

[`complex-not-nan`](../../r7rs/types/complex-not-nan.md#type__r7rs__complex-not-nan);
[`real`](../../r7rs/types/real.md#type__r7rs__real);


##### Super-types recursive

[`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);
[`number`](../../r7rs/types/number.md#type__r7rs__number);
[`complex`](../../r7rs/types/complex.md#type__r7rs__complex);


#### Sub-types

[`real-not-inf-not-nan`](../../r7rs/types/real-not-inf-not-nan.md#type__r7rs__real-not-inf-not-nan);
[`real-not-zero-not-nan`](../../r7rs/types/real-not-zero-not-nan.md#type__r7rs__real-not-zero-not-nan);
[`real-positive-or-zero`](../../r7rs/types/real-positive-or-zero.md#type__r7rs__real-positive-or-zero);
[`real-negative-or-zero`](../../r7rs/types/real-negative-or-zero.md#type__r7rs__real-negative-or-zero);
[`inexact-real-not-nan`](../../r7rs/types/inexact-real-not-nan.md#type__r7rs__inexact-real-not-nan);


##### Sub-types recursive

[`rational`](../../r7rs/types/rational.md#type__r7rs__rational);
[`integer`](../../r7rs/types/integer.md#type__r7rs__integer);
[`exact-real`](../../r7rs/types/exact-real.md#type__r7rs__exact-real);
[`exact-rational`](../../r7rs/types/exact-rational.md#type__r7rs__exact-rational);
[`exact-integer`](../../r7rs/types/exact-integer.md#type__r7rs__exact-integer);
[`inexact-rational`](../../r7rs/types/inexact-rational.md#type__r7rs__inexact-rational);
[`inexact-integer`](../../r7rs/types/inexact-integer.md#type__r7rs__inexact-integer);
[`real-zero`](../../r7rs/types/real-zero.md#type__r7rs__real-zero);
[`rational-zero`](../../r7rs/types/rational-zero.md#type__r7rs__rational-zero);
[`integer-zero`](../../r7rs/types/integer-zero.md#type__r7rs__integer-zero);
[`exact-integer-zero`](../../r7rs/types/exact-integer-zero.md#type__r7rs__exact-integer-zero);
[`rational-not-zero`](../../r7rs/types/rational-not-zero.md#type__r7rs__rational-not-zero);
[`integer-not-zero`](../../r7rs/types/integer-not-zero.md#type__r7rs__integer-not-zero);
[`exact-integer-not-zero`](../../r7rs/types/exact-integer-not-zero.md#type__r7rs__exact-integer-not-zero);
[`real-inf`](../../r7rs/types/real-inf.md#type__r7rs__real-inf);
[`real-positive`](../../r7rs/types/real-positive.md#type__r7rs__real-positive);
[`real-positive-not-inf`](../../r7rs/types/real-positive-not-inf.md#type__r7rs__real-positive-not-inf);
[`rational-positive`](../../r7rs/types/rational-positive.md#type__r7rs__rational-positive);
[`integer-positive`](../../r7rs/types/integer-positive.md#type__r7rs__integer-positive);
[`exact-integer-positive`](../../r7rs/types/exact-integer-positive.md#type__r7rs__exact-integer-positive);
[`real-negative`](../../r7rs/types/real-negative.md#type__r7rs__real-negative);
[`real-negative-not-inf`](../../r7rs/types/real-negative-not-inf.md#type__r7rs__real-negative-not-inf);
[`rational-negative`](../../r7rs/types/rational-negative.md#type__r7rs__rational-negative);
[`integer-negative`](../../r7rs/types/integer-negative.md#type__r7rs__integer-negative);
[`exact-integer-negative`](../../r7rs/types/exact-integer-negative.md#type__r7rs__exact-integer-negative);
[`real-positive-or-zero-not-inf`](../../r7rs/types/real-positive-or-zero-not-inf.md#type__r7rs__real-positive-or-zero-not-inf);
[`rational-positive-or-zero`](../../r7rs/types/rational-positive-or-zero.md#type__r7rs__rational-positive-or-zero);
[`integer-positive-or-zero`](../../r7rs/types/integer-positive-or-zero.md#type__r7rs__integer-positive-or-zero);
[`exact-integer-positive-or-zero`](../../r7rs/types/exact-integer-positive-or-zero.md#type__r7rs__exact-integer-positive-or-zero);
[`real-negative-or-zero-not-inf`](../../r7rs/types/real-negative-or-zero-not-inf.md#type__r7rs__real-negative-or-zero-not-inf);
[`rational-negative-or-zero`](../../r7rs/types/rational-negative-or-zero.md#type__r7rs__rational-negative-or-zero);
[`integer-negative-or-zero`](../../r7rs/types/integer-negative-or-zero.md#type__r7rs__integer-negative-or-zero);
[`exact-integer-negative-or-zero`](../../r7rs/types/exact-integer-negative-or-zero.md#type__r7rs__exact-integer-negative-or-zero);
[`inexact-real-not-inf-not-nan`](../../r7rs/types/inexact-real-not-inf-not-nan.md#type__r7rs__inexact-real-not-inf-not-nan);
[`code-point-unicode`](../../r7rs/types/code-point-unicode.md#type__r7rs__code-point-unicode);
[`code-point-ascii`](../../r7rs/types/code-point-ascii.md#type__r7rs__code-point-ascii);
[`range-value`](../../r7rs/types/range-value.md#type__r7rs__range-value);
[`range-offset`](../../r7rs/types/range-offset.md#type__r7rs__range-offset);
[`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
[`range-end`](../../r7rs/types/range-end.md#type__r7rs__range-end);
[`range-length`](../../r7rs/types/range-length.md#type__r7rs__range-length);
[`range-length-zero`](../../r7rs/types/range-length-zero.md#type__r7rs__range-length-zero);
[`range-length-not-zero`](../../r7rs/types/range-length-not-zero.md#type__r7rs__range-length-not-zero);
[`byte`](../../r7rs/types/byte.md#type__r7rs__byte);
[`byte-ascii`](../../r7rs/types/byte-ascii.md#type__r7rs__byte-ascii);


#### Referent definitions as input

[`exp`](../../r7rs/definitions/exp.md#definition__r7rs__exp);
[`sin`](../../r7rs/definitions/sin.md#definition__r7rs__sin);
[`cos`](../../r7rs/definitions/cos.md#definition__r7rs__cos);
[`tan`](../../r7rs/definitions/tan.md#definition__r7rs__tan);
[`asin`](../../r7rs/definitions/asin.md#definition__r7rs__asin);
[`acos`](../../r7rs/definitions/acos.md#definition__r7rs__acos);
[`atan`](../../r7rs/definitions/atan.md#definition__r7rs__atan);


#### Referent definitions as input (recursive)

[`number?`](../../r7rs/definitions/number_3f.md#definition__r7rs__number_3f);
[`integer?`](../../r7rs/definitions/integer_3f.md#definition__r7rs__integer_3f);
[`real?`](../../r7rs/definitions/real_3f.md#definition__r7rs__real_3f);
[`rational?`](../../r7rs/definitions/rational_3f.md#definition__r7rs__rational_3f);
[`complex?`](../../r7rs/definitions/complex_3f.md#definition__r7rs__complex_3f);
[`exact?`](../../r7rs/definitions/exact_3f.md#definition__r7rs__exact_3f);
[`inexact?`](../../r7rs/definitions/inexact_3f.md#definition__r7rs__inexact_3f);
[`exact-integer?`](../../r7rs/definitions/exact-integer_3f.md#definition__r7rs__exact-integer_3f);
[`zero?`](../../r7rs/definitions/zero_3f.md#definition__r7rs__zero_3f);
[`positive?`](../../r7rs/definitions/positive_3f.md#definition__r7rs__positive_3f);
[`negative?`](../../r7rs/definitions/negative_3f.md#definition__r7rs__negative_3f);
[`odd?`](../../r7rs/definitions/odd_3f.md#definition__r7rs__odd_3f);
[`even?`](../../r7rs/definitions/even_3f.md#definition__r7rs__even_3f);
[`=`](../../r7rs/definitions/ZZZZ__3d.md#definition__r7rs__ZZZZ__3d);
[`<`](../../r7rs/definitions/ZZZZ__3c.md#definition__r7rs__ZZZZ__3c);
[`>`](../../r7rs/definitions/ZZZZ__3e.md#definition__r7rs__ZZZZ__3e);
[`<=`](../../r7rs/definitions/ZZZZ__3c_3d.md#definition__r7rs__ZZZZ__3c_3d);
[`>=`](../../r7rs/definitions/ZZZZ__3e_3d.md#definition__r7rs__ZZZZ__3e_3d);
[`+`](../../r7rs/definitions/ZZZZ__2b.md#definition__r7rs__ZZZZ__2b);
[`-`](../../r7rs/definitions/ZZZZ__2d.md#definition__r7rs__ZZZZ__2d);
[`*`](../../r7rs/definitions/ZZZZ__2a.md#definition__r7rs__ZZZZ__2a);
[`/`](../../r7rs/definitions/ZZZZ__2f.md#definition__r7rs__ZZZZ__2f);
[`floor/`](../../r7rs/definitions/floor_2f.md#definition__r7rs__floor_2f);
[`floor-quotient`](../../r7rs/definitions/floor-quotient.md#definition__r7rs__floor-quotient);
[`floor-remainder`](../../r7rs/definitions/floor-remainder.md#definition__r7rs__floor-remainder);
[`truncate/`](../../r7rs/definitions/truncate_2f.md#definition__r7rs__truncate_2f);
[`truncate-quotient`](../../r7rs/definitions/truncate-quotient.md#definition__r7rs__truncate-quotient);
[`truncate-remainder`](../../r7rs/definitions/truncate-remainder.md#definition__r7rs__truncate-remainder);
[`min`](../../r7rs/definitions/min.md#definition__r7rs__min);
[`max`](../../r7rs/definitions/max.md#definition__r7rs__max);
[`gcd`](../../r7rs/definitions/gcd.md#definition__r7rs__gcd);
[`lcm`](../../r7rs/definitions/lcm.md#definition__r7rs__lcm);
[`expt`](../../r7rs/definitions/expt.md#definition__r7rs__expt);
[`square`](../../r7rs/definitions/square.md#definition__r7rs__square);
[`inexact`](../../r7rs/definitions/inexact.md#definition__r7rs__inexact);
[`number->string`](../../r7rs/definitions/number-_3e_string.md#definition__r7rs__number-_3e_string);

Note:  These definitions consume an input that is a super-type.


#### Referent definitions as output

[`exp`](../../r7rs/definitions/exp.md#definition__r7rs__exp);
[`log`](../../r7rs/definitions/log.md#definition__r7rs__log);
[`sin`](../../r7rs/definitions/sin.md#definition__r7rs__sin);
[`cos`](../../r7rs/definitions/cos.md#definition__r7rs__cos);
[`tan`](../../r7rs/definitions/tan.md#definition__r7rs__tan);
[`asin`](../../r7rs/definitions/asin.md#definition__r7rs__asin);
[`acos`](../../r7rs/definitions/acos.md#definition__r7rs__acos);
[`atan`](../../r7rs/definitions/atan.md#definition__r7rs__atan);


#### Referent definitions as output (recursive)

[`rationalize`](../../r7rs/definitions/rationalize.md#definition__r7rs__rationalize);
[`floor`](../../r7rs/definitions/floor.md#definition__r7rs__floor);
[`ceiling`](../../r7rs/definitions/ceiling.md#definition__r7rs__ceiling);
[`truncate`](../../r7rs/definitions/truncate.md#definition__r7rs__truncate);
[`round`](../../r7rs/definitions/round.md#definition__r7rs__round);
[`numerator`](../../r7rs/definitions/numerator.md#definition__r7rs__numerator);
[`denominator`](../../r7rs/definitions/denominator.md#definition__r7rs__denominator);
[`make-rectangular`](../../r7rs/definitions/make-rectangular.md#definition__r7rs__make-rectangular);
[`real-part`](../../r7rs/definitions/real-part.md#definition__r7rs__real-part);
[`imag-part`](../../r7rs/definitions/imag-part.md#definition__r7rs__imag-part);
[`make-polar`](../../r7rs/definitions/make-polar.md#definition__r7rs__make-polar);
[`angle`](../../r7rs/definitions/angle.md#definition__r7rs__angle);
[`sqrt`](../../r7rs/definitions/sqrt.md#definition__r7rs__sqrt);
[`magnitude`](../../r7rs/definitions/magnitude.md#definition__r7rs__magnitude);
[`digit-value`](../../r7rs/definitions/digit-value.md#definition__r7rs__digit-value);
[`char->integer`](../../r7rs/definitions/char-_3e_integer.md#definition__r7rs__char-_3e_integer);
[`length`](../../r7rs/definitions/length.md#definition__r7rs__length);
[`vector-length`](../../r7rs/definitions/vector-length.md#definition__r7rs__vector-length);
[`string-length`](../../r7rs/definitions/string-length.md#definition__r7rs__string-length);
[`bytevector-length`](../../r7rs/definitions/bytevector-length.md#definition__r7rs__bytevector-length);
[`bytevector-u8-ref`](../../r7rs/definitions/bytevector-u8-ref.md#definition__r7rs__bytevector-u8-ref);

Note:  These definitions produce an output that is a sub-type.


#### Predicate

```
(|lambda| (|value|) (|and| (|real?| |value|) (|not| (|nan?| |value|))))
```


#### Categories

[`r7rs:types-numbers`](../../r7rs/categories/r7rs_3a_types-numbers.md#category__r7rs__r7rs_3a_types-numbers);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----
