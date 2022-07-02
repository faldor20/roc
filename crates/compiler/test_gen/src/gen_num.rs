#[cfg(feature = "gen-llvm")]
use crate::helpers::llvm::assert_evals_to;

#[cfg(feature = "gen-dev")]
use crate::helpers::dev::assert_evals_to;

#[cfg(feature = "gen-wasm")]
use crate::helpers::wasm::assert_evals_to;

// use crate::assert_wasm_evals_to as assert_evals_to;
#[allow(unused_imports)]
use indoc::indoc;
#[allow(unused_imports)]
use roc_std::{RocDec, RocOrder};

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev", feature = "gen-wasm"))]
fn nat_alias() {
    assert_evals_to!(
        indoc!(
            r#"
                    i : Num.Nat
                    i = 1

                    i
                "#
        ),
        1,
        usize
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn i128_signed_int_alias() {
    assert_evals_to!(
        indoc!(
            r#"
                    i : I128
                    i = 128

                    i
                "#
        ),
        128,
        i128
    );
}
#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev", feature = "gen-wasm"))]
fn i64_signed_int_alias() {
    assert_evals_to!(
        indoc!(
            r#"
                app "test" provides [main] to "./platform"

                main =
                    i : I64
                    i = 64

                    i
                "#
        ),
        64,
        i64
    );
}
#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev", feature = "gen-wasm"))]
fn i32_signed_int_alias() {
    assert_evals_to!(
        indoc!(
            r#"
                    i : I32
                    i = 32

                    i
                "#
        ),
        32,
        i32
    );
}
#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev", feature = "gen-wasm"))]
fn i16_signed_int_alias() {
    assert_evals_to!(
        indoc!(
            r#"
                    i : I16
                    i = 16

                    i
                "#
        ),
        16,
        i16
    );
}
#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev", feature = "gen-wasm"))]
fn i8_signed_int_alias() {
    assert_evals_to!(
        indoc!(
            r#"
                    i : I8
                    i = 8

                    i
                "#
        ),
        8,
        i8
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn i128_hex_int_alias() {
    assert_evals_to!(
        indoc!(
            r#"
                    f : I128
                    f = 0x123

                    f
                "#
        ),
        0x123,
        i128
    );
}
#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev", feature = "gen-wasm"))]
fn i64_hex_int_alias() {
    assert_evals_to!(
        indoc!(
            r#"
                    f : I64
                    f = 0x123

                    f
                "#
        ),
        0x123,
        i64
    );
}
#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev", feature = "gen-wasm"))]
fn i32_hex_int_alias() {
    assert_evals_to!(
        indoc!(
            r#"
                    f : I32
                    f = 0x123

                    f
                "#
        ),
        0x123,
        i32
    );
}
#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev", feature = "gen-wasm"))]
fn i16_hex_int_alias() {
    assert_evals_to!(
        indoc!(
            r#"
                    f : I16
                    f = 0x123

                    f
                "#
        ),
        0x123,
        i16
    );
}
#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev", feature = "gen-wasm"))]
fn i8_hex_int_alias() {
    assert_evals_to!(
        indoc!(
            r#"
                    f : I8
                    f = 0xA

                    f
                "#
        ),
        0xA,
        i8
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn u128_signed_int_alias() {
    assert_evals_to!(
        indoc!(
            r#"
                    i : U128
                    i = 128

                    i
                "#
        ),
        128,
        u128
    );
}
#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev", feature = "gen-wasm"))]
fn u64_signed_int_alias() {
    assert_evals_to!(
        indoc!(
            r#"
                    i : U64
                    i = 64

                    i
                "#
        ),
        64,
        u64
    );
}
#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev", feature = "gen-wasm"))]
fn u32_signed_int_alias() {
    assert_evals_to!(
        indoc!(
            r#"
                    i : U32
                    i = 32

                    i
                "#
        ),
        32,
        u32
    );
}
#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev", feature = "gen-wasm"))]
fn u16_signed_int_alias() {
    assert_evals_to!(
        indoc!(
            r#"
                    i : U16
                    i = 16

                    i
                "#
        ),
        16,
        u16
    );
}
#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev", feature = "gen-wasm"))]
fn u8_signed_int_alias() {
    assert_evals_to!(
        indoc!(
            r#"
                    i : U8
                    i = 8

                    i
                "#
        ),
        8,
        u8
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn u128_hex_int_alias() {
    assert_evals_to!(
        indoc!(
            r#"
                    f : U128
                    f = 0x123

                    f
                "#
        ),
        0x123,
        i128
    );
}
#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev", feature = "gen-wasm"))]
fn u64_hex_int_alias() {
    assert_evals_to!(
        indoc!(
            r#"
                    f : U64
                    f = 0x123

                    f
                "#
        ),
        0x123,
        u64
    );
}
#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev", feature = "gen-wasm"))]
fn u32_hex_int_alias() {
    assert_evals_to!(
        indoc!(
            r#"
                    f : U32
                    f = 0x123

                    f
                "#
        ),
        0x123,
        u32
    );
}
#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev", feature = "gen-wasm"))]
fn u16_hex_int_alias() {
    assert_evals_to!(
        indoc!(
            r#"
                    f : U16
                    f = 0x123

                    f
                "#
        ),
        0x123,
        u16
    );
}
#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev", feature = "gen-wasm"))]
fn u8_hex_int_alias() {
    assert_evals_to!(
        indoc!(
            r#"
                    f : U8
                    f = 0xA

                    f
                "#
        ),
        0xA,
        u8
    );
}

#[test]
fn character_literal() {
    assert_evals_to!(
        indoc!(
            r#"
                    x = 'A'

                    x
                "#
        ),
        65,
        u32
    );
}

#[test]
fn character_literal_back_slash() {
    assert_evals_to!(
        indoc!(
            r#"
                    x = '\\'

                    x
                "#
        ),
        92,
        u32
    );
}

#[test]
fn character_literal_single_quote() {
    assert_evals_to!(
        indoc!(
            r#"
                    x = '\''

                    x
                "#
        ),
        39,
        u32
    );
}

#[test]
fn character_literal_new_line() {
    assert_evals_to!(
        indoc!(
            r#"
                    x = '\n'

                    x
                "#
        ),
        10,
        u32
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn dec_float_alias() {
    assert_evals_to!(
        indoc!(
            r#"
                    x : Dec
                    x = 2.1

                    x
                "#
        ),
        RocDec::from_str_to_i128_unsafe("2.1"),
        i128
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev", feature = "gen-wasm"))]
fn f64_float_alias() {
    assert_evals_to!(
        indoc!(
            r#"
                    f : F64
                    f = 3.6

                    f
                "#
        ),
        3.6,
        f64
    );
}
#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn f32_float_alias() {
    assert_evals_to!(
        indoc!(
            r#"
                    f : F32
                    f = 3.6

                    f
                "#
        ),
        3.6,
        f32
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn f64_sqrt() {
    assert_evals_to!(
        indoc!(
            r#"
                    when Num.sqrtChecked 100 is
                        Ok val -> val
                        Err _ -> -1
                "#
        ),
        10.0,
        f64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn f64_log() {
    assert_evals_to!(
        indoc!(
            r#"
                    Num.log 7.38905609893
                "#
        ),
        1.999999999999912,
        f64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn f64_log_checked_one() {
    assert_evals_to!(
        indoc!(
            r#"
                    when Num.logChecked 1 is
                        Ok val -> val
                        Err _ -> -1
                "#
        ),
        0.0,
        f64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn f64_sqrt_zero() {
    assert_evals_to!(
        indoc!(
            r#"
                    when Num.sqrtChecked 0 is
                        Ok val -> val
                        Err _ -> -1
                "#
        ),
        0.0,
        f64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn f64_sqrt_checked_negative() {
    assert_evals_to!(
        indoc!(
            r#"
                    when Num.sqrtChecked -1 is
                        Err _ -> 42
                        Ok val -> val
                "#
        ),
        42.0,
        f64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn f64_log_checked_zero() {
    assert_evals_to!(
        indoc!(
            r#"
                    when Num.logChecked 0 is
                        Err _ -> 42
                        Ok val -> val
                "#
        ),
        42.0,
        f64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn f64_log_negative() {
    assert_evals_to!(
        indoc!(
            r#"
                    Num.log -1
                "#
        ),
        true,
        f64,
        |f: f64| f.is_nan()
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev", feature = "gen-wasm"))]
fn f64_round() {
    assert_evals_to!("Num.round 3.6", 4, i64);
    assert_evals_to!("Num.round 3.4", 3, i64);
    assert_evals_to!("Num.round 2.5", 3, i64);
    assert_evals_to!("Num.round -2.3", -2, i64);
    assert_evals_to!("Num.round -2.5", -3, i64);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev", feature = "gen-wasm"))]
fn f64_abs() {
    assert_evals_to!("Num.abs -4.7", 4.7, f64);
    assert_evals_to!("Num.abs 5.8", 5.8, f64);

    #[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
    {
        assert_evals_to!("Num.abs Num.maxF64", f64::MAX, f64);
        assert_evals_to!("Num.abs Num.minF64", f64::MAX, f64);
    }
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn i64_abs() {
    assert_evals_to!("Num.abs -6", 6, i64);
    assert_evals_to!("Num.abs 7", 7, i64);
    assert_evals_to!("Num.abs 0", 0, i64);
    assert_evals_to!("Num.abs -0", 0, i64);
    assert_evals_to!("Num.abs -1", 1, i64);
    assert_evals_to!("Num.abs 1", 1, i64);
    assert_evals_to!("Num.abs 9_000_000_000_000", 9_000_000_000_000, i64);
    assert_evals_to!("Num.abs -9_000_000_000_000", 9_000_000_000_000, i64);
    assert_evals_to!("Num.abs Num.maxI64", i64::MAX, i64);
    assert_evals_to!("Num.abs (Num.minI64 + 1)", -(i64::MIN + 1), i64);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
#[should_panic(
    expected = r#"Roc failed with message: "integer absolute overflowed because its argument is the minimum value"#
)]
fn abs_min_int_overflow() {
    assert_evals_to!(
        indoc!(
            r#"
                Num.abs Num.minI64
                "#
        ),
        0,
        i64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev", feature = "gen-wasm"))]
fn gen_if_fn() {
    assert_evals_to!(
        indoc!(
            r#"
                    limitedNegate = \num ->
                        x =
                            if num == 1 then
                                -1
                            else if num == -1 then
                                1
                            else
                                num
                        x

                    limitedNegate 1
                "#
        ),
        -1,
        i64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn gen_float_eq() {
    assert_evals_to!(
        indoc!(
            r#"
                    1.0 == 1.0
                "#
        ),
        true,
        bool
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn gen_add_dec() {
    assert_evals_to!(
        indoc!(
            r#"
                    x : Dec
                    x = 2.1

                    y : Dec
                    y = 3.1

                    z : Dec
                    z = x + y

                    z
                "#
        ),
        RocDec::from_str_to_i128_unsafe("5.2"),
        i128
    );
}
#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev", feature = "gen-wasm"))]
fn gen_add_f64() {
    assert_evals_to!(
        indoc!(
            r#"
                    1.1 + 2.4 + 3
                "#
        ),
        6.5,
        f64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev", feature = "gen-wasm"))]
fn gen_wrap_add_nums() {
    assert_evals_to!(
        indoc!(
            r#"
                    add2 = \num1, num2 -> num1 + num2

                    add2 4 5
                "#
        ),
        9,
        i64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn gen_div_f64() {
    assert_evals_to!(
        indoc!(
            r#"
                    48 / 2
                "#
        ),
        24.0,
        f64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn gen_div_checked_f64() {
    assert_evals_to!(
        indoc!(
            r#"
                    when Num.divChecked 48 2 is
                        Ok val -> val
                        Err _ -> -1
                "#
        ),
        24.0,
        f64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn gen_div_checked_by_zero_f64() {
    assert_evals_to!(
        indoc!(
            r#"
                    when Num.divChecked 47 0 is
                        Ok val -> val
                        Err _ -> -1
                "#
        ),
        -1.0,
        f64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn gen_div_dec() {
    assert_evals_to!(
        indoc!(
            r#"
                    x : Dec
                    x = 10

                    y : Dec
                    y = 3

                    x / y
                "#
        ),
        RocDec::from_str_to_i128_unsafe("3.333333333333333333"),
        i128
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn gen_div_checked_dec() {
    assert_evals_to!(
        indoc!(
            r#"
                    x : Dec
                    x = 10

                    y : Dec
                    y = 3

                    when Num.divChecked x y is
                        Ok val -> val
                        Err _ -> -1
                "#
        ),
        RocDec::from_str_to_i128_unsafe("3.333333333333333333"),
        i128
    );
}
#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn gen_div_checked_by_zero_dec() {
    assert_evals_to!(
        indoc!(
            r#"
                    x : Dec
                    x = 10

                    y : Dec
                    y = 0

                    when Num.divChecked x y is
                        Ok val -> val
                        Err _ -> -1
                "#
        ),
        RocDec::from_str_to_i128_unsafe("-1"),
        i128
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev", feature = "gen-wasm"))]
fn gen_int_eq() {
    assert_evals_to!(
        indoc!(
            r#"
                    4 == 4
                "#
        ),
        true,
        bool
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm", feature = "gen-dev"))]
fn gen_int_neq() {
    assert_evals_to!(
        indoc!(
            r#"
                    4 != 5
                "#
        ),
        true,
        bool
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm", feature = "gen-dev"))]
fn gen_int_less_than() {
    assert_evals_to!(
        indoc!(
            r#"
                    4 < 5
                "#
        ),
        true,
        bool
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn gen_dec_eq() {
    assert_evals_to!(
        indoc!(
            r#"
                    x : Dec
                    x = 4

                    y : Dec
                    y = 4

                    x == y
                "#
        ),
        true,
        bool
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn gen_dec_neq() {
    assert_evals_to!(
        indoc!(
            r#"
                    x : Dec
                    x = 4

                    y : Dec
                    y = 5

                    x != y
                "#
        ),
        true,
        bool
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn gen_wrap_int_neq() {
    assert_evals_to!(
        indoc!(
            r#"
                    wrappedNotEq : a, a -> Bool
                    wrappedNotEq = \num1, num2 ->
                        num1 != num2

                    wrappedNotEq 2 3
                "#
        ),
        true,
        bool
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev", feature = "gen-wasm"))]
fn gen_add_i64() {
    assert_evals_to!(
        indoc!(
            r#"
                    1 + 2 + 3
                "#
        ),
        6,
        i64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn gen_sub_dec() {
    assert_evals_to!(
        indoc!(
            r#"
                    x : Dec
                    x = 1.5

                    y : Dec
                    y = 2.4

                    z : Dec
                    z = 3

                    (x - y) - z
                "#
        ),
        RocDec::from_str_to_i128_unsafe("-3.9"),
        i128
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn gen_sub_f64() {
    assert_evals_to!(
        indoc!(
            r#"
                    1.5 - 2.4 - 3
                "#
        ),
        -3.9,
        f64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev", feature = "gen-wasm"))]
fn gen_sub_i64() {
    assert_evals_to!(
        indoc!(
            r#"
                    1 - 2 - 3
                "#
        ),
        -4,
        i64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn gen_mul_dec() {
    assert_evals_to!(
        indoc!(
            r#"
                    x : Dec
                    x = 2

                    y : Dec
                    y = 4

                    z : Dec
                    z = 6

                    x * y * z
                "#
        ),
        RocDec::from_str_to_i128_unsafe("48.0"),
        i128
    );
}
#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev", feature = "gen-wasm"))]
fn gen_mul_i64() {
    assert_evals_to!(
        indoc!(
            r#"
                    2 * 4 * 6
                "#
        ),
        48,
        i64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn gen_div_i64() {
    assert_evals_to!(
        indoc!(
            r#"
                    1000 // 10
                "#
        ),
        100,
        i64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn gen_div_checked_i64() {
    assert_evals_to!(
        indoc!(
            r#"
                    when Num.divTruncChecked 1000 10 is
                        Ok val -> val
                        Err _ -> -1
                "#
        ),
        100,
        i64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn gen_div_checked_by_zero_i64() {
    assert_evals_to!(
        indoc!(
            r#"
                    when Num.divTruncChecked 1000 0 is
                        Err DivByZero -> 99
                        _ -> -24
                "#
        ),
        99,
        i64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn gen_rem_i64() {
    assert_evals_to!(
        indoc!(
            r#"
                    Num.rem 8 3
                "#
        ),
        2,
        i64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn gen_rem_checked_div_by_zero_i64() {
    assert_evals_to!(
        indoc!(
            r#"
                    when Num.remChecked 8 0 is
                        Err DivByZero -> 4
                        Ok _ -> -23
                "#
        ),
        4,
        i64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm", feature = "gen-dev"))]
fn gen_is_zero_i64() {
    assert_evals_to!("Num.isZero 0", true, bool);
    assert_evals_to!("Num.isZero 1", false, bool);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn gen_is_positive_i64() {
    assert_evals_to!("Num.isPositive 0", false, bool);
    assert_evals_to!("Num.isPositive 1", true, bool);
    assert_evals_to!("Num.isPositive -5", false, bool);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn gen_is_negative_i64() {
    assert_evals_to!("Num.isNegative 0", false, bool);
    assert_evals_to!("Num.isNegative 3", false, bool);
    assert_evals_to!("Num.isNegative -2", true, bool);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn gen_is_positive_f64() {
    assert_evals_to!("Num.isPositive 0.0", false, bool);
    assert_evals_to!("Num.isPositive 4.7", true, bool);
    assert_evals_to!("Num.isPositive -8.5", false, bool);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn gen_is_negative_f64() {
    assert_evals_to!("Num.isNegative 0.0", false, bool);
    assert_evals_to!("Num.isNegative 9.9", false, bool);
    assert_evals_to!("Num.isNegative -4.4", true, bool);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn gen_is_zero_f64() {
    assert_evals_to!("Num.isZero 0", true, bool);
    assert_evals_to!("Num.isZero 0_0", true, bool);
    assert_evals_to!("Num.isZero 0.0", true, bool);
    assert_evals_to!("Num.isZero 1", false, bool);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn gen_is_odd() {
    assert_evals_to!("Num.isOdd 4", false, bool);
    assert_evals_to!("Num.isOdd 5", true, bool);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn gen_is_even() {
    assert_evals_to!("Num.isEven 6", true, bool);
    assert_evals_to!("Num.isEven 7", false, bool);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn sin() {
    assert_evals_to!("Num.sin 0", 0.0, f64);
    assert_evals_to!("Num.sin 1.41421356237", 0.9877659459922529, f64);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn cos() {
    assert_evals_to!("Num.cos 0", 1.0, f64);
    assert_evals_to!("Num.cos 3.14159265359", -1.0, f64);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn tan() {
    assert_evals_to!("Num.tan 0", 0.0, f64);
    assert_evals_to!("Num.tan 1", 1.557407724654902, f64);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn bitwise_and() {
    assert_evals_to!("Num.bitwiseAnd 20 20", 20, i64);
    assert_evals_to!("Num.bitwiseAnd 25 10", 8, i64);
    assert_evals_to!("Num.bitwiseAnd 200 0", 0, i64);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn bitwise_xor() {
    assert_evals_to!("Num.bitwiseXor 20 20", 0, i64);
    assert_evals_to!("Num.bitwiseXor 15 14", 1, i64);
    assert_evals_to!("Num.bitwiseXor 7 15", 8, i64);
    assert_evals_to!("Num.bitwiseXor 200 0", 200, i64);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn bitwise_or() {
    assert_evals_to!("Num.bitwiseOr 1 1", 1, i64);
    assert_evals_to!("Num.bitwiseOr 1 2", 3, i64);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn lt_u8() {
    assert_evals_to!("1u8 < 2u8", true, bool);
    assert_evals_to!("1u8 < 1u8", false, bool);
    assert_evals_to!("2u8 < 1u8", false, bool);
    assert_evals_to!("0u8 < 0u8", false, bool);
    assert_evals_to!("128u8 < 0u8", false, bool);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn lte_u8() {
    assert_evals_to!("1u8 <= 1u8", true, bool);
    assert_evals_to!("2u8 <= 1u8", false, bool);
    assert_evals_to!("1u8 <= 2u8", true, bool);
    assert_evals_to!("0u8 <= 0u8", true, bool);
    assert_evals_to!("128u8 <= 0u8", false, bool);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn gt_u8() {
    assert_evals_to!("2u8 > 1u8", true, bool);
    assert_evals_to!("2u8 > 2u8", false, bool);
    assert_evals_to!("1u8 > 1u8", false, bool);
    assert_evals_to!("0u8 > 0u8", false, bool);
    assert_evals_to!("0u8 > 128u8", false, bool);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn gte_u8() {
    assert_evals_to!("1u8 >= 1u8", true, bool);
    assert_evals_to!("1u8 >= 2u8", false, bool);
    assert_evals_to!("2u8 >= 1u8", true, bool);
    assert_evals_to!("0u8 >= 0u8", true, bool);
    assert_evals_to!("0u8 >= 128u8", false, bool);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn lt_u64() {
    assert_evals_to!("1u64 < 2u64", true, bool);
    assert_evals_to!("1u64 < 1u64", false, bool);
    assert_evals_to!("2u64 < 1u64", false, bool);
    assert_evals_to!("0u64 < 0u64", false, bool);
    assert_evals_to!("9223372036854775808u64 < 0u64", false, bool);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn lte_u64() {
    assert_evals_to!("1u64 <= 1u64", true, bool);
    assert_evals_to!("2u64 <= 1u64", false, bool);
    assert_evals_to!("1u64 <= 2u64", true, bool);
    assert_evals_to!("0u64 <= 0u64", true, bool);
    assert_evals_to!("9223372036854775808u64 <= 0u64", false, bool);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn gt_u64() {
    assert_evals_to!("2u64 > 1u64", true, bool);
    assert_evals_to!("2u64 > 2u64", false, bool);
    assert_evals_to!("1u64 > 1u64", false, bool);
    assert_evals_to!("0u64 > 0u64", false, bool);
    assert_evals_to!("0u64 > 9223372036854775808u64", false, bool);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn gte_u64() {
    assert_evals_to!("1u64 >= 1u64", true, bool);
    assert_evals_to!("1u64 >= 2u64", false, bool);
    assert_evals_to!("2u64 >= 1u64", true, bool);
    assert_evals_to!("0u64 >= 0u64", true, bool);
    assert_evals_to!("0u64 >= 9223372036854775808u64", false, bool);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn lt_i64() {
    assert_evals_to!("1 < 2", true, bool);
    assert_evals_to!("1 < 1", false, bool);
    assert_evals_to!("2 < 1", false, bool);
    assert_evals_to!("0 < 0", false, bool);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn lte_i64() {
    assert_evals_to!("1 <= 1", true, bool);
    assert_evals_to!("2 <= 1", false, bool);
    assert_evals_to!("1 <= 2", true, bool);
    assert_evals_to!("0 <= 0", true, bool);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn gt_i64() {
    assert_evals_to!("2 > 1", true, bool);
    assert_evals_to!("2 > 2", false, bool);
    assert_evals_to!("1 > 1", false, bool);
    assert_evals_to!("0 > 0", false, bool);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn gte_i64() {
    assert_evals_to!("1 >= 1", true, bool);
    assert_evals_to!("1 >= 2", false, bool);
    assert_evals_to!("2 >= 1", true, bool);
    assert_evals_to!("0 >= 0", true, bool);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn lt_f64() {
    assert_evals_to!("1.1 < 1.2", true, bool);
    assert_evals_to!("1.1 < 1.1", false, bool);
    assert_evals_to!("1.2 < 1.1", false, bool);
    assert_evals_to!("0.0 < 0.0", false, bool);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn lte_f64() {
    assert_evals_to!("1.1 <= 1.1", true, bool);
    assert_evals_to!("1.2 <= 1.1", false, bool);
    assert_evals_to!("1.1 <= 1.2", true, bool);
    assert_evals_to!("0.0 <= 0.0", true, bool);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn gt_f64() {
    assert_evals_to!("2.2 > 1.1", true, bool);
    assert_evals_to!("2.2 > 2.2", false, bool);
    assert_evals_to!("1.1 > 2.2", false, bool);
    assert_evals_to!("0.0 > 0.0", false, bool);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn gte_f64() {
    assert_evals_to!("1.1 >= 1.1", true, bool);
    assert_evals_to!("1.1 >= 1.2", false, bool);
    assert_evals_to!("1.2 >= 1.1", true, bool);
    assert_evals_to!("0.0 >= 0.0", true, bool);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev", feature = "gen-wasm"))]
fn gen_order_of_arithmetic_ops() {
    assert_evals_to!(
        indoc!(
            r#"
                    1 + 3 * 7 - 2
                "#
        ),
        20,
        i64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn gen_order_of_arithmetic_ops_complex_float() {
    assert_evals_to!(
        indoc!(
            r#"
                    3 - 48 * 2.0
                "#
        ),
        -93.0,
        f64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev", feature = "gen-wasm"))]
fn if_guard_bind_variable_false() {
    assert_evals_to!(
        indoc!(
            r#"
                wrapper = \{} ->
                    when 10 is
                        x if x == 5 -> 0
                        _ -> 42

                wrapper {}
                "#
        ),
        42,
        i64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev", feature = "gen-wasm"))]
fn if_guard_bind_variable_true() {
    assert_evals_to!(
        indoc!(
            r#"
                wrapper = \{} ->
                    when 10 is
                        x if x == 10 -> 42
                        _ -> 0

                wrapper {}
                "#
        ),
        42,
        i64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev", feature = "gen-wasm"))]
fn tail_call_elimination() {
    assert_evals_to!(
        indoc!(
            r#"
                    sum = \n, accum ->
                        when n is
                            0 -> accum
                            _ -> sum (n - 1) (n + accum)

                    sum 1_000_000 0
                "#
        ),
        500000500000,
        i64
    );
}

#[test]
#[cfg(any(feature = "gen-dev"))]
fn int_negate_dev() {
    // TODO
    // dev backend yet to have `Num.maxI64` or `Num.minI64`.
    // add the "gen-dev" feature to the test below after implementing them both.
    assert_evals_to!("Num.neg 123", -123, i64);
    assert_evals_to!("Num.neg -123", 123, i64);
    assert_evals_to!("Num.neg 0", 0, i64);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn int_negate() {
    assert_evals_to!("Num.neg 123", -123, i64);
    assert_evals_to!("Num.neg Num.maxI64", -i64::MAX, i64);
    assert_evals_to!("Num.neg (Num.minI64 + 1)", i64::MAX, i64);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
#[should_panic(
    expected = r#"Roc failed with message: "integer negation overflowed because its argument is the minimum value"#
)]
fn neg_min_int_overflow() {
    assert_evals_to!(
        indoc!(
            r#"
                Num.neg Num.minI64
                "#
        ),
        0,
        i64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn gen_wrap_int_neg() {
    assert_evals_to!(
        indoc!(
            r#"
                    wrappedNeg = \num -> -num

                    wrappedNeg 3
                "#
        ),
        -3,
        i64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev", feature = "gen-wasm"))]
fn gen_basic_fn() {
    assert_evals_to!(
        indoc!(
            r#"
                    always42 : Num.Num (Num.Integer Num.Signed64) -> Num.Num (Num.Integer Num.Signed64)
                    always42 = \_ -> 42

                    always42 5
                "#
        ),
        42,
        i64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm", feature = "gen-dev"))]
fn int_to_float() {
    assert_evals_to!("Num.toFrac 0x9", 9.0, f64);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm", feature = "gen-dev"))]
fn num_to_frac() {
    assert_evals_to!("Num.toFrac 9", 9.0, f64);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn num_to_frac_f64_to_f32() {
    assert_evals_to!(
        indoc!(
            r#"
                    f64 : F64
                    f64 = 9.0

                    f32 : F32
                    f32 = Num.toFrac f64
                    f32
                "#
        ),
        9.0,
        f32
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn num_to_frac_f32_to_f32() {
    assert_evals_to!(
        indoc!(
            r#"

                    arg : F32
                    arg = 9.0

                    ret : F32
                    ret = Num.toFrac arg
                    ret
                "#
        ),
        9.0,
        f32
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm", feature = "gen-dev"))]
fn num_to_frac_f64_to_f64() {
    assert_evals_to!(
        indoc!(
            r#"

                    arg : F64
                    arg = 9.0

                    ret : F64
                    ret = Num.toFrac arg
                    ret
                "#
        ),
        9.0,
        f64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm", feature = "gen-dev"))]
fn num_to_frac_f32_to_f64() {
    assert_evals_to!(
        indoc!(
            r#"

                    f32 : F32
                    f32 = 9.0

                    f64 : F64
                    f64 = Num.toFrac f32
                    f64
                "#
        ),
        9.0,
        f64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm", feature = "gen-dev"))]
fn float_to_float() {
    assert_evals_to!("Num.toFrac 0.5", 0.5, f64);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn int_compare() {
    assert_evals_to!("Num.compare 0 1", RocOrder::Lt, RocOrder);
    assert_evals_to!("Num.compare 1 1", RocOrder::Eq, RocOrder);
    assert_evals_to!("Num.compare 1 0", RocOrder::Gt, RocOrder);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn float_compare() {
    assert_evals_to!("Num.compare 0.01 3.14", RocOrder::Lt, RocOrder);
    assert_evals_to!("Num.compare 3.14 3.14", RocOrder::Eq, RocOrder);
    assert_evals_to!("Num.compare 3.14 0.01", RocOrder::Gt, RocOrder);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn pow() {
    assert_evals_to!("Num.pow 2.0 2.0", 4.0, f64);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn ceiling() {
    assert_evals_to!("Num.ceiling 1.1", 2, i64);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn floor() {
    assert_evals_to!("Num.floor 1.9", 1, i64);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev", feature = "gen-wasm"))]
fn pow_int() {
    assert_evals_to!("Num.powInt 2 3", 8, i64);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev", feature = "gen-wasm"))]
fn atan() {
    assert_evals_to!("Num.atan 10", 1.4711276743037347, f64);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
#[should_panic(expected = r#"Roc failed with message: "integer addition overflowed!"#)]
fn int_add_overflow() {
    assert_evals_to!(
        indoc!(
            r#"
                9_223_372_036_854_775_807 + 1
                "#
        ),
        0,
        i64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn int_add_checked() {
    assert_evals_to!(
        indoc!(
            r#"
                when Num.addChecked 1 2 is
                    Ok v -> v
                    _ -> -1
                "#
        ),
        3,
        i64
    );

    assert_evals_to!(
        indoc!(
            r#"
                when Num.addChecked 9_223_372_036_854_775_807 1 is
                    Err Overflow -> -1
                    Ok v -> v
                "#
        ),
        -1,
        i64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn int_add_wrap() {
    assert_evals_to!(
        indoc!(
            r#"
                Num.addWrap 9_223_372_036_854_775_807 1
                "#
        ),
        std::i64::MIN,
        i64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn float_add_checked_pass() {
    assert_evals_to!(
        indoc!(
            r#"
                when Num.addChecked 1.0 0.0 is
                    Ok v -> v
                    Err Overflow -> -1.0
                "#
        ),
        1.0,
        f64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn float_add_checked_fail() {
    assert_evals_to!(
        indoc!(
            r#"
                when Num.addChecked 1.7976931348623157e308 1.7976931348623157e308 is
                    Err Overflow -> -1
                    Ok v -> v
                "#
        ),
        -1.0,
        f64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn float_add_overflow() {
    assert_evals_to!(
        indoc!(
            r#"
                    1.7976931348623157e308 + 1.7976931348623157e308
                    "#
        ),
        f64::INFINITY,
        f64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
#[should_panic(expected = r#"Roc failed with message: "integer subtraction overflowed!"#)]
fn int_sub_overflow() {
    assert_evals_to!(
        indoc!(
            r#"
                -9_223_372_036_854_775_808 - 1
                "#
        ),
        0,
        i64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn int_sub_wrap() {
    assert_evals_to!(
        indoc!(
            r#"
                Num.subWrap -9_223_372_036_854_775_808 1
                "#
        ),
        std::i64::MAX,
        i64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn float_sub_overflow() {
    assert_evals_to!(
        indoc!(
            r#"
                    -1.7976931348623157e308 - 1.7976931348623157e308
                "#
        ),
        -f64::INFINITY,
        f64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn int_sub_checked() {
    assert_evals_to!(
        indoc!(
            r#"
                when Num.subChecked 5 2 is
                    Ok v -> v
                    _ -> -1
                "#
        ),
        3,
        i64
    );

    assert_evals_to!(
        indoc!(
            r#"
                when Num.subChecked Num.minI64 1 is
                    Err Overflow -> -1
                    Ok v -> v
                "#
        ),
        -1,
        i64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn float_sub_checked() {
    assert_evals_to!(
        indoc!(
            r#"
                when Num.subChecked 1.0 0.0 is
                    Ok v -> v
                    Err Overflow -> -1.0
                "#
        ),
        1.0,
        f64
    );

    assert_evals_to!(
        indoc!(
            r#"
                when Num.subChecked -1.7976931348623157e308 1.7976931348623157e308 is
                    Err Overflow -> -1
                    Ok v -> v
                "#
        ),
        -1.0,
        f64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
#[should_panic(expected = r#"Roc failed with message: "integer multiplication overflowed!"#)]
fn int_positive_mul_overflow() {
    assert_evals_to!(
        indoc!(
            r#"
                9_223_372_036_854_775_807 * 2
                "#
        ),
        0,
        i64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
#[should_panic(expected = r#"Roc failed with message: "integer multiplication overflowed!"#)]
fn int_negative_mul_overflow() {
    assert_evals_to!(
        indoc!(
            r#"
                (-9_223_372_036_854_775_808) * 2
                "#
        ),
        0,
        i64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn float_positive_mul_overflow() {
    assert_evals_to!(
        indoc!(
            r#"
                    1.7976931348623157e308 * 2
                "#
        ),
        f64::INFINITY,
        f64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn float_negative_mul_overflow() {
    assert_evals_to!(
        indoc!(
            r#"
                    -1.7976931348623157e308 * 2
                "#
        ),
        -f64::INFINITY,
        f64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn int_mul_wrap() {
    assert_evals_to!(
        indoc!(
            r#"
                Num.mulWrap Num.maxI64 2
                "#
        ),
        -2,
        i64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn int_mul_checked() {
    assert_evals_to!(
        indoc!(
            r#"
                when Num.mulChecked 20 2 is
                    Ok v -> v
                    _ -> -1
                "#
        ),
        40,
        i64
    );

    assert_evals_to!(
        indoc!(
            r#"
                when Num.mulChecked Num.maxI64 2 is
                    Err Overflow -> -1
                    Ok v -> v
                "#
        ),
        -1,
        i64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn float_mul_checked() {
    assert_evals_to!(
        indoc!(
            r#"
                when Num.mulChecked 20.0 2.0 is
                    Ok v -> v
                    Err Overflow -> -1.0
                "#
        ),
        40.0,
        f64
    );

    assert_evals_to!(
        indoc!(
            r#"
                when Num.mulChecked 1.7976931348623157e308 2 is
                    Err Overflow -> -1
                    Ok v -> v
                "#
        ),
        -1.0,
        f64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn shift_left_by() {
    assert_evals_to!("Num.shiftLeftBy 0 0b0000_0001", 0b0000_0001, i64);
    assert_evals_to!("Num.shiftLeftBy 1 0b0000_0001", 0b0000_0010, i64);
    assert_evals_to!("Num.shiftLeftBy 2 0b0000_0011", 0b0000_1100, i64);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn shift_right_by() {
    // Sign Extended Right Shift

    let is_wasm = cfg!(feature = "gen-wasm");
    let is_llvm_release_mode = cfg!(feature = "gen-llvm") && !cfg!(debug_assertions);

    // FIXME (Brian) Something funny happening with 8-bit binary literals in tests
    if !is_wasm {
        assert_evals_to!(
            "Num.shiftRightBy 2 (Num.toI8 0b1100_0000u8)",
            0b1111_0000u8 as i8,
            i8
        );
        assert_evals_to!("Num.shiftRightBy 2 0b0100_0000i8", 0b0001_0000i8, i8);
        assert_evals_to!("Num.shiftRightBy 1 0b1110_0000u8", 0b1111_0000u8, u8);
        assert_evals_to!("Num.shiftRightBy 2 0b1100_0000u8", 0b1111_0000u8, u8);
        assert_evals_to!("Num.shiftRightBy 12 0b0100_0000u8", 0b0000_0000u8, u8);

        // LLVM in release mode returns 0 instead of -1 for some reason
        if !is_llvm_release_mode {
            assert_evals_to!("Num.shiftRightBy 12 0b1000_0000u8", 0b1111_1111u8, u8);
        }
    }
    assert_evals_to!("Num.shiftRightBy 0 12", 12, i64);
    assert_evals_to!("Num.shiftRightBy 1 12", 6, i64);
    assert_evals_to!("Num.shiftRightBy 1 -12", -6, i64);
    assert_evals_to!("Num.shiftRightBy 8 12", 0, i64);
    assert_evals_to!("Num.shiftRightBy 8 -12", -1, i64);
    assert_evals_to!("Num.shiftRightBy -1 12", 0, i64);
    assert_evals_to!("Num.shiftRightBy 0 0", 0, i64);
    assert_evals_to!("Num.shiftRightBy 1 0", 0, i64);

    assert_evals_to!("Num.shiftRightBy 0 12i32", 12, i32);
    assert_evals_to!("Num.shiftRightBy 1 12i32", 6, i32);
    assert_evals_to!("Num.shiftRightBy 1 -12i32", -6, i32);
    assert_evals_to!("Num.shiftRightBy 8 12i32", 0, i32);
    assert_evals_to!("Num.shiftRightBy 8 -12i32", -1, i32);

    assert_evals_to!("Num.shiftRightBy 0 12i8", 12, i8);
    assert_evals_to!("Num.shiftRightBy 1 12i8", 6, i8);
    assert_evals_to!("Num.shiftRightBy 1 -12i8", -6, i8);
    assert_evals_to!("Num.shiftRightBy 8 12i8", 0, i8);

    if !is_llvm_release_mode {
        assert_evals_to!("Num.shiftRightBy -1 0", 0, i64);
        assert_evals_to!("Num.shiftRightBy -1 -12", -1, i64);
        assert_evals_to!("Num.shiftRightBy 8 -12i8", -1, i8);
    }
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn shift_right_zf_by() {
    // Logical Right Shift
    assert_evals_to!(
        "Num.shiftRightZfBy 2 (Num.toI8 0b1100_0000u8)",
        0b0011_0000i8,
        i8
    );
    assert_evals_to!("Num.shiftRightZfBy 2 0b1100_0000u8", 0b0011_0000u8, u8);
    assert_evals_to!("Num.shiftRightZfBy 1 0b0000_0010u8", 0b0000_0001u8, u8);
    assert_evals_to!("Num.shiftRightZfBy 2 0b0000_1100u8", 0b0000_0011u8, u8);
    assert_evals_to!("Num.shiftRightZfBy 12 0b1000_0000u8", 0b0000_0000u8, u8);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn min_i128() {
    assert_evals_to!(
        indoc!(
            r#"
                Num.minI128
                "#
        ),
        i128::MIN,
        i128
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn max_i128() {
    assert_evals_to!(
        indoc!(
            r#"
                Num.maxI128
                "#
        ),
        i128::MAX,
        i128
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn min_i64() {
    assert_evals_to!(
        indoc!(
            r#"
                Num.minI64
                "#
        ),
        i64::MIN,
        i64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn max_i64() {
    assert_evals_to!(
        indoc!(
            r#"
                Num.maxI64
                "#
        ),
        i64::MAX,
        i64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn min_u64() {
    assert_evals_to!(
        indoc!(
            r#"
                Num.minU64
                "#
        ),
        u64::MIN,
        u64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn max_u64() {
    assert_evals_to!(
        indoc!(
            r#"
                Num.maxU64
                "#
        ),
        u64::MAX,
        u64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn min_i32() {
    assert_evals_to!(
        indoc!(
            r#"
                Num.minI32
                "#
        ),
        i32::MIN,
        i32
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn max_i32() {
    assert_evals_to!(
        indoc!(
            r#"
                Num.maxI32
                "#
        ),
        i32::MAX,
        i32
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn min_u32() {
    assert_evals_to!(
        indoc!(
            r#"
                Num.minU32
                "#
        ),
        u32::MIN,
        u32
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn max_u32() {
    assert_evals_to!(
        indoc!(
            r#"
                Num.maxU32
                "#
        ),
        u32::MAX,
        u32
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn min_i16() {
    assert_evals_to!(
        indoc!(
            r#"
                Num.minI16
                "#
        ),
        i16::MIN,
        i16
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn max_i16() {
    assert_evals_to!(
        indoc!(
            r#"
                Num.maxI16
                "#
        ),
        i16::MAX,
        i16
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn min_u16() {
    assert_evals_to!(
        indoc!(
            r#"
                Num.minU16
                "#
        ),
        u16::MIN,
        u16
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn max_u16() {
    assert_evals_to!(
        indoc!(
            r#"
                Num.maxU16
                "#
        ),
        u16::MAX,
        u16
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn min_i8() {
    assert_evals_to!(
        indoc!(
            r#"
                Num.minI8
                "#
        ),
        i8::MIN,
        i8
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn max_i8() {
    assert_evals_to!(
        indoc!(
            r#"
                Num.maxI8
                "#
        ),
        i8::MAX,
        i8
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn min_u8() {
    assert_evals_to!(
        indoc!(
            r#"
                Num.minU8
                "#
        ),
        u8::MIN,
        u8
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn max_u8() {
    assert_evals_to!(
        indoc!(
            r#"
                Num.maxU8
                "#
        ),
        u8::MAX,
        u8
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn max_f64() {
    assert_evals_to!(
        indoc!(
            r#"
            Num.maxF64
            "#
        ),
        f64::MAX,
        f64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn min_f64() {
    assert_evals_to!(
        indoc!(
            r#"
            Num.minF64
            "#
        ),
        f64::MIN,
        f64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn max_f32() {
    assert_evals_to!(
        indoc!(
            r#"
            Num.maxF32
            "#
        ),
        f32::MAX,
        f32
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn min_f32() {
    assert_evals_to!(
        indoc!(
            r#"
            Num.minF32
            "#
        ),
        f32::MIN,
        f32
    );
}

macro_rules! num_conversion_tests {
    ($($fn:expr, $typ:ty, ($($test_name:ident, $input:expr, $output:expr $(, [$($support_gen:literal),*])? )*))*) => {$($(
        #[test]
        #[cfg(any(feature = "gen-llvm", $($(feature = $support_gen)*)?))]
        fn $test_name() {
            let input = format!("{} {}", $fn, $input);
            assert_evals_to!(&input, $output, $typ)
        }
    )*)*}
}

num_conversion_tests! {
    "Num.toI8", i8, (
        to_i8_same_width, "15u8", 15, ["gen-wasm"]
        to_i8_truncate, "115i32", 115, ["gen-wasm"]
        to_i8_truncate_wraps, "500i32", -12, ["gen-wasm"]
    )
    "Num.toI16", i16, (
        to_i16_same_width, "15u16", 15, ["gen-wasm"]
        to_i16_extend, "15i8", 15, ["gen-wasm"]
        to_i16_truncate, "115i32", 115, ["gen-wasm"]
        to_i16_truncate_wraps, "60000i32", -5536, ["gen-wasm"]
    )
    "Num.toI32", i32, (
        to_i32_same_width, "15u32", 15, ["gen-wasm"]
        to_i32_extend, "15i8", 15, ["gen-wasm"]
        to_i32_truncate, "115i64", 115, ["gen-wasm"]
        to_i32_truncate_wraps, "5000000000i64", 705032704, ["gen-wasm"]
    )
    "Num.toI64", i64, (
        to_i64_same_width, "15u64", 15, ["gen-wasm"]
        to_i64_extend, "15i8", 15, ["gen-wasm"]
        to_i64_truncate, "115i128", 115
        to_i64_truncate_wraps, "10_000_000_000_000_000_000i128", -8446744073709551616
    )
    "Num.toI128", i128, (
        to_i128_same_width, "15u128", 15
        to_i128_extend, "15i8", 15
    )
    "Num.toU8", u8, (
        to_u8_same_width, "15i8", 15, ["gen-wasm"]
        to_u8_truncate, "115i32", 115, ["gen-wasm"]
        to_u8_truncate_wraps, "500i32", 244, ["gen-wasm"]
    )
    "Num.toU16", u16, (
        to_u16_same_width, "15i16", 15, ["gen-wasm"]
        to_u16_extend, "15i8", 15, ["gen-wasm"]
        to_u16_truncate, "115i32", 115, ["gen-wasm"]
        to_u16_truncate_wraps, "600000000i32", 17920, ["gen-wasm"]
    )
    "Num.toU32", u32, (
        to_u32_same_width, "15i32", 15, ["gen-wasm"]
        to_u32_extend, "15i8", 15, ["gen-wasm"]
        to_u32_truncate, "115i64", 115, ["gen-wasm"]
        to_u32_truncate_wraps, "5000000000000000000i64", 1156841472, ["gen-wasm"]
    )
    "Num.toU64", u64, (
        to_u64_same_width, "15i64", 15, ["gen-wasm"]
        to_u64_extend, "15i8", 15, ["gen-wasm"]
        to_u64_truncate, "115i128", 115
        to_u64_truncate_wraps, "10_000_000_000_000_000_000_000i128", 1864712049423024128
    )
    "Num.toU128", u128, (
        to_u128_same_width, "15i128", 15
        to_u128_extend, "15i8", 15
    )
    "Num.toNat", usize, (
        to_nat_same_width, "15i64", 15, ["gen-wasm"]
        to_nat_extend, "15i8", 15, ["gen-wasm"]
        to_nat_truncate, "115i128", 115
        to_nat_truncate_wraps, "10_000_000_000_000_000_000_000i128", 1864712049423024128
    )
    "Num.toF32", f32, (
        to_f32_from_i8, "15i8", 15.0
        to_f32_from_i16, "15i16", 15.0
        to_f32_from_i32, "15i32", 15.0
        to_f32_from_i64, "15i64", 15.0
        to_f32_from_i128, "15i128", 15.0
        to_f32_from_u8, "15u8", 15.0
        to_f32_from_u16, "15u16", 15.0
        to_f32_from_u32, "15u32", 15.0
        to_f32_from_u64, "15u64", 15.0
        to_f32_from_u128, "15u128", 15.0
        to_f32_from_nat, "15nat", 15.0
        to_f32_from_f32, "1.5f32", 1.5
        to_f32_from_f64, "1.5f64", 1.5
    )
    "Num.toF64", f64, (
        to_f64_from_i8, "15i8", 15.0
        to_f64_from_i16, "15i16", 15.0
        to_f64_from_i32, "15i32", 15.0
        to_f64_from_i64, "15i64", 15.0
        to_f64_from_i128, "15i128", 15.0
        to_f64_from_u8, "15u8", 15.0
        to_f64_from_u16, "15u16", 15.0
        to_f64_from_u32, "15u32", 15.0
        to_f64_from_u64, "15u64", 15.0
        to_f64_from_u128, "15u128", 15.0
        to_f64_from_nat, "15nat", 15.0
        to_f64_from_f32, "1.5f32", 1.5
        to_f64_from_f64, "1.5f64", 1.5
    )
}

macro_rules! to_int_checked_tests {
    ($($fn:expr, $typ:ty, ($($test_name:ident, $input:expr, $output:expr)*))*) => {$($(
        #[test]
        #[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
        fn $test_name() {
            let sentinel = 23;
            // Some n = Ok n, None = OutOfBounds
            let expected = match $output.into() {
                None => sentinel,
                Some(n) => {
                    assert_ne!(n, sentinel);
                    n
                }
            };
            let input = format!("Result.withDefault ({} {}) {}", $fn, $input, sentinel);
            assert_evals_to!(&input, expected, $typ)
        }
    )*)*}
}

to_int_checked_tests! {
    "Num.toI8Checked", i8, (
        to_i8_checked_same,                             "15i8",    15
        to_i8_checked_same_width_unsigned_fits,         "15u8",    15
        to_i8_checked_same_width_unsigned_oob,          "128u8",   None
        to_i8_checked_larger_width_signed_fits_pos,     "15i16",   15
        to_i8_checked_larger_width_signed_oob_pos,      "128i16",  None
        to_i8_checked_larger_width_signed_fits_neg,     "-15i16",  -15
        to_i8_checked_larger_width_signed_oob_neg,      "-129i16", None
        to_i8_checked_larger_width_unsigned_fits_pos,   "15u16",   15
        to_i8_checked_larger_width_unsigned_oob_pos,    "128u16",  None
    )
    "Num.toI16Checked", i16, (
        to_i16_checked_smaller_width_pos,                "15i8",      15
        to_i16_checked_smaller_width_neg,                "-15i8",     -15
        to_i16_checked_same,                             "15i16",     15
        to_i16_checked_same_width_unsigned_fits,         "15u16",     15
        to_i16_checked_same_width_unsigned_oob,          "32768u16",  None
        to_i16_checked_larger_width_signed_fits_pos,     "15i32",     15
        to_i16_checked_larger_width_signed_oob_pos,      "32768i32",  None
        to_i16_checked_larger_width_signed_fits_neg,     "-15i32",    -15
        to_i16_checked_larger_width_signed_oob_neg,      "-32769i32", None
        to_i16_checked_larger_width_unsigned_fits_pos,   "15u32",     15
        to_i16_checked_larger_width_unsigned_oob_pos,    "32768u32",  None
    )
    "Num.toI32Checked", i32, (
        to_i32_checked_smaller_width_pos,                "15i8",      15
        to_i32_checked_smaller_width_neg,                "-15i8",     -15
        to_i32_checked_same,                             "15i32",     15
        to_i32_checked_same_width_unsigned_fits,         "15u32",     15
        to_i32_checked_same_width_unsigned_oob,          "2147483648u32",  None
        to_i32_checked_larger_width_signed_fits_pos,     "15i64",     15
        to_i32_checked_larger_width_signed_oob_pos,      "2147483648i64",  None
        to_i32_checked_larger_width_signed_fits_neg,     "-15i64",    -15
        to_i32_checked_larger_width_signed_oob_neg,      "-2147483649i64", None
        to_i32_checked_larger_width_unsigned_fits_pos,   "15u64",     15
        to_i32_checked_larger_width_unsigned_oob_pos,    "2147483648u64",  None
    )
    "Num.toI64Checked", i64, (
        to_i64_checked_smaller_width_pos,                "15i8",      15
        to_i64_checked_smaller_width_neg,                "-15i8",     -15
        to_i64_checked_same,                             "15i64",     15
        to_i64_checked_same_width_unsigned_fits,         "15u64",     15
        to_i64_checked_same_width_unsigned_oob,          "9223372036854775808u64",  None
        to_i64_checked_larger_width_signed_fits_pos,     "15i128",     15
        to_i64_checked_larger_width_signed_oob_pos,      "9223372036854775808i128",  None
        to_i64_checked_larger_width_signed_fits_neg,     "-15i128",    -15
        to_i64_checked_larger_width_signed_oob_neg,      "-9223372036854775809i128", None
        to_i64_checked_larger_width_unsigned_fits_pos,   "15u128",     15
        to_i64_checked_larger_width_unsigned_oob_pos,    "9223372036854775808u128",  None
    )
    "Num.toI128Checked", i128, (
        to_i128_checked_smaller_width_pos,                "15i8",      15
        to_i128_checked_smaller_width_neg,                "-15i8",     -15
        to_i128_checked_same,                             "15i128",     15
        to_i128_checked_same_width_unsigned_fits,         "15u128",     15
        to_i128_checked_same_width_unsigned_oob,          "170141183460469231731687303715884105728u128",  None
    )
    "Num.toU8Checked", u8, (
        to_u8_checked_same,                           "15u8",   15
        to_u8_checked_same_width_signed_fits,         "15i8",   15
        to_u8_checked_same_width_signed_oob,          "-1i8",   None
        to_u8_checked_larger_width_signed_fits_pos,   "15i16",  15
        to_u8_checked_larger_width_signed_oob_pos,    "256i16", None
        to_u8_checked_larger_width_signed_oob_neg,    "-1i16",  None
        to_u8_checked_larger_width_unsigned_fits_pos, "15u16",  15
        to_u8_checked_larger_width_unsigned_oob_pos,  "256u16", None
    )
    "Num.toU16Checked", u16, (
        to_u16_checked_smaller_width_pos,              "15i8",     15
        to_u16_checked_smaller_width_neg_oob,          "-15i8",    None
        to_u16_checked_same,                           "15u16",    15
        to_u16_checked_same_width_signed_fits,         "15i16",    15
        to_u16_checked_same_width_signed_oob,          "-1i16",    None
        to_u16_checked_larger_width_signed_fits_pos,   "15i32",    15
        to_u16_checked_larger_width_signed_oob_pos,    "65536i32", None
        to_u16_checked_larger_width_signed_oob_neg,    "-1i32",    None
        to_u16_checked_larger_width_unsigned_fits_pos, "15u32",    15
        to_u16_checked_larger_width_unsigned_oob_pos,  "65536u32", None
    )
    "Num.toU32Checked", u32, (
        to_u32_checked_smaller_width_pos,              "15i8",     15
        to_u32_checked_smaller_width_neg_oob,          "-15i8",    None
        to_u32_checked_same,                           "15u32",    15
        to_u32_checked_same_width_signed_fits,         "15i32",    15
        to_u32_checked_same_width_signed_oob,          "-1i32",    None
        to_u32_checked_larger_width_signed_fits_pos,   "15i64",    15
        to_u32_checked_larger_width_signed_oob_pos,    "4294967296i64", None
        to_u32_checked_larger_width_signed_oob_neg,    "-1i64",    None
        to_u32_checked_larger_width_unsigned_fits_pos, "15u64",    15
        to_u32_checked_larger_width_unsigned_oob_pos,  "4294967296u64", None
    )
    "Num.toU64Checked", u64, (
        to_u64_checked_smaller_width_pos,              "15i8",     15
        to_u64_checked_smaller_width_neg_oob,          "-15i8",    None
        to_u64_checked_same,                           "15u64",    15
        to_u64_checked_same_width_signed_fits,         "15i64",    15
        to_u64_checked_same_width_signed_oob,          "-1i64",    None
        to_u64_checked_larger_width_signed_fits_pos,   "15i128",   15
        to_u64_checked_larger_width_signed_oob_pos,    "18446744073709551616i128", None
        to_u64_checked_larger_width_signed_oob_neg,    "-1i128",   None
        to_u64_checked_larger_width_unsigned_fits_pos, "15u128",   15
        to_u64_checked_larger_width_unsigned_oob_pos,  "18446744073709551616u128", None
    )
    "Num.toU128Checked", u128, (
        to_u128_checked_smaller_width_pos,             "15i8",     15
        to_u128_checked_smaller_width_neg_oob,         "-15i8",    None
        to_u128_checked_same,                          "15u128",   15
        to_u128_checked_same_width_signed_fits,        "15i128",   15
        to_u128_checked_same_width_signed_oob,         "-1i128",   None
    )
    "Num.toNatChecked", usize, (
        to_nat_checked_smaller_width_pos,              "15i8",     15
        to_nat_checked_smaller_width_neg_oob,          "-15i8",    None
        to_nat_checked_same,                           "15u64",    15
        to_nat_checked_same_width_signed_fits,         "15i64",    15
        to_nat_checked_same_width_signed_oob,          "-1i64",    None
        to_nat_checked_larger_width_signed_fits_pos,   "15i128",   15
        to_nat_checked_larger_width_signed_oob_pos,    "18446744073709551616i128", None
        to_nat_checked_larger_width_signed_oob_neg,    "-1i128",   None
        to_nat_checked_larger_width_unsigned_fits_pos, "15u128",   15
        to_nat_checked_larger_width_unsigned_oob_pos,  "18446744073709551616u128", None
    )
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn is_multiple_of_signed() {
    // true
    assert_evals_to!("Num.isMultipleOf 5 1", true, bool);
    assert_evals_to!("Num.isMultipleOf 5 -1", true, bool);
    assert_evals_to!("Num.isMultipleOf 0 0", true, bool);
    assert_evals_to!("Num.isMultipleOf 0 1", true, bool);
    assert_evals_to!("Num.isMultipleOf 0 -1", true, bool);
    // false
    assert_evals_to!("Num.isMultipleOf 5 2", false, bool);
    assert_evals_to!("Num.isMultipleOf 5 0", false, bool);

    // overflow
    assert_evals_to!("Num.isMultipleOf -9223372036854775808 -1", true, bool);
}

#[test]
#[cfg(any(feature = "gen-wasm"))]
fn is_multiple_of_unsigned() {
    // true
    assert_evals_to!("Num.isMultipleOf 5u8 1", true, bool);
    assert_evals_to!("Num.isMultipleOf 0u8 0", true, bool);
    assert_evals_to!("Num.isMultipleOf 0u8 1", true, bool);
    assert_evals_to!("Num.isMultipleOf 0u8 0xFF", true, bool);

    // false
    assert_evals_to!("Num.isMultipleOf 5u8 2", false, bool);
    assert_evals_to!("Num.isMultipleOf 5u8 0", false, bool);

    // unsigned result is different from signed
    assert_evals_to!("Num.isMultipleOf 5u8 0xFF", false, bool);
    assert_evals_to!("Num.isMultipleOf 0xFCu8 0xFE", false, bool);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn bytes_to_u16_clearly_out_of_bounds() {
    assert_evals_to!(
        indoc!(
            r#"
                bytes = Str.toUtf8 "hello"
                when Num.bytesToU16 bytes 234 is
                    Ok v -> v
                    Err OutOfBounds -> 1
                "#
        ),
        1,
        u16
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn bytes_to_u16_subtly_out_of_bounds() {
    assert_evals_to!(
        indoc!(
            r#"
                bytes = Str.toUtf8 "hello"
                when Num.bytesToU16 bytes 4 is
                    Ok v -> v
                    Err OutOfBounds -> 1
                "#
        ),
        1,
        u16
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn bytes_to_u32_clearly_out_of_bounds() {
    assert_evals_to!(
        indoc!(
            r#"
                bytes = Str.toUtf8 "hello"
                when Num.bytesToU32 bytes 234 is
                    Ok v -> v
                    Err OutOfBounds -> 1
                "#
        ),
        1,
        u32
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn bytes_to_u32_subtly_out_of_bounds() {
    assert_evals_to!(
        indoc!(
            r#"
                bytes = Str.toUtf8 "hello"
                when Num.bytesToU32 bytes 2 is
                    Ok v -> v
                    Err OutOfBounds -> 1
                "#
        ),
        1,
        u32
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn bytes_to_u16_max_u8s() {
    assert_evals_to!(
        indoc!(
            r#"
                when Num.bytesToU16 [255, 255] 0 is
                    Ok v -> v
                    Err OutOfBounds -> 1
                "#
        ),
        65535,
        u16
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn bytes_to_u16_min_u8s() {
    assert_evals_to!(
        indoc!(
            r#"
                when Num.bytesToU16 [0, 0] 0 is
                    Ok v -> v
                    Err OutOfBounds -> 1
                "#
        ),
        0,
        u16
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn bytes_to_u16_random_u8s() {
    assert_evals_to!(
        indoc!(
            r#"
                when Num.bytesToU16 [164, 215] 0 is
                    Ok v -> v
                    Err OutOfBounds -> 1
                "#
        ),
        55_204,
        u16
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn bytes_to_u32_min_u8s() {
    assert_evals_to!(
        indoc!(
            r#"
                when Num.bytesToU32 [0, 0, 0, 0] 0 is
                    Ok v -> v
                    Err OutOfBounds -> 1
                "#
        ),
        0,
        u32
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn bytes_to_u32_max_u8s() {
    assert_evals_to!(
        indoc!(
            r#"
                when Num.bytesToU32 [255, 255, 255, 255] 0 is
                    Ok v -> v
                    Err OutOfBounds -> 1
                "#
        ),
        4_294_967_295,
        u32
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn bytes_to_u32_random_u8s() {
    assert_evals_to!(
        indoc!(
            r#"
                when Num.bytesToU32 [252, 124, 128, 121] 0 is
                    Ok v -> v
                    Err OutOfBounds -> 1
                "#
        ),
        2_038_463_740,
        u32
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn when_on_i32() {
    assert_evals_to!(
        indoc!(
            r#"
                app "test" provides [main] to "./platform"

                x : I32
                x = 0

                main : I32
                main =
                    when x is
                        0 -> 42
                        _ -> -1
            "#
        ),
        42,
        i32
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn when_on_i16() {
    assert_evals_to!(
        indoc!(
            r#"
                app "test" provides [main] to "./platform"

                x : I16
                x = 0

                main : I16
                main =
                    when x is
                        0 -> 42
                        _ -> -1
            "#
        ),
        42,
        i16
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn num_to_str() {
    use roc_std::RocStr;

    assert_evals_to!(r#"Num.toStr 1234"#, RocStr::from("1234"), RocStr);
    assert_evals_to!(r#"Num.toStr 0"#, RocStr::from("0"), RocStr);
    assert_evals_to!(r#"Num.toStr -1"#, RocStr::from("-1"), RocStr);

    let max = format!("{}", i64::MAX);
    assert_evals_to!(
        r#"Num.toStr Num.maxI64"#,
        RocStr::from(max.as_str()),
        RocStr
    );

    let min = format!("{}", i64::MIN);
    assert_evals_to!(
        r#"Num.toStr Num.minI64"#,
        RocStr::from(min.as_str()),
        RocStr
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn num_to_str_u8() {
    use roc_std::RocStr;

    assert_evals_to!(r#"Num.toStr 0u8"#, RocStr::from("0"), RocStr);
    assert_evals_to!(r#"Num.toStr 1u8"#, RocStr::from("1"), RocStr);
    assert_evals_to!(r#"Num.toStr 10u8"#, RocStr::from("10"), RocStr);

    let max = format!("{}", u8::MAX);
    assert_evals_to!(r#"Num.toStr Num.maxU8"#, RocStr::from(max.as_str()), RocStr);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn num_to_str_u16() {
    use roc_std::RocStr;

    assert_evals_to!(r#"Num.toStr 0u16"#, RocStr::from("0"), RocStr);
    assert_evals_to!(r#"Num.toStr 1u16"#, RocStr::from("1"), RocStr);
    assert_evals_to!(r#"Num.toStr 10u16"#, RocStr::from("10"), RocStr);

    let max = format!("{}", u16::MAX);
    assert_evals_to!(
        r#"Num.toStr Num.maxU16"#,
        RocStr::from(max.as_str()),
        RocStr
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn num_to_str_u32() {
    use roc_std::RocStr;

    assert_evals_to!(r#"Num.toStr 0u32"#, RocStr::from("0"), RocStr);
    assert_evals_to!(r#"Num.toStr 1u32"#, RocStr::from("1"), RocStr);
    assert_evals_to!(r#"Num.toStr 10u32"#, RocStr::from("10"), RocStr);

    let max = format!("{}", u32::MAX);
    assert_evals_to!(
        r#"Num.toStr Num.maxU32"#,
        RocStr::from(max.as_str()),
        RocStr
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn num_to_str_u64() {
    use roc_std::RocStr;

    assert_evals_to!(r#"Num.toStr 0u64"#, RocStr::from("0"), RocStr);
    assert_evals_to!(r#"Num.toStr 1u64"#, RocStr::from("1"), RocStr);
    assert_evals_to!(r#"Num.toStr 10u64"#, RocStr::from("10"), RocStr);

    let max = format!("{}", u64::MAX);
    assert_evals_to!(
        r#"Num.toStr Num.maxU64"#,
        RocStr::from(max.as_str()),
        RocStr
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn num_to_str_i8() {
    use roc_std::RocStr;

    assert_evals_to!(r#"Num.toStr -10i8"#, RocStr::from("-10"), RocStr);
    assert_evals_to!(r#"Num.toStr -1i8"#, RocStr::from("-1"), RocStr);
    assert_evals_to!(r#"Num.toStr 0i8"#, RocStr::from("0"), RocStr);
    assert_evals_to!(r#"Num.toStr 1i8"#, RocStr::from("1"), RocStr);
    assert_evals_to!(r#"Num.toStr 10i8"#, RocStr::from("10"), RocStr);

    let max = format!("{}", i8::MAX);
    assert_evals_to!(r#"Num.toStr Num.maxI8"#, RocStr::from(max.as_str()), RocStr);

    let max = format!("{}", i8::MIN);
    assert_evals_to!(r#"Num.toStr Num.minI8"#, RocStr::from(max.as_str()), RocStr);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn num_to_str_i16() {
    use roc_std::RocStr;

    assert_evals_to!(r#"Num.toStr -10i16"#, RocStr::from("-10"), RocStr);
    assert_evals_to!(r#"Num.toStr -1i16"#, RocStr::from("-1"), RocStr);
    assert_evals_to!(r#"Num.toStr 0i16"#, RocStr::from("0"), RocStr);
    assert_evals_to!(r#"Num.toStr 1i16"#, RocStr::from("1"), RocStr);
    assert_evals_to!(r#"Num.toStr 10i16"#, RocStr::from("10"), RocStr);

    let max = format!("{}", i16::MAX);
    assert_evals_to!(
        r#"Num.toStr Num.maxI16"#,
        RocStr::from(max.as_str()),
        RocStr
    );

    let max = format!("{}", i16::MIN);
    assert_evals_to!(
        r#"Num.toStr Num.minI16"#,
        RocStr::from(max.as_str()),
        RocStr
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn num_to_str_i32() {
    use roc_std::RocStr;

    assert_evals_to!(r#"Num.toStr -10i32"#, RocStr::from("-10"), RocStr);
    assert_evals_to!(r#"Num.toStr -1i32"#, RocStr::from("-1"), RocStr);
    assert_evals_to!(r#"Num.toStr 0i32"#, RocStr::from("0"), RocStr);
    assert_evals_to!(r#"Num.toStr 1i32"#, RocStr::from("1"), RocStr);
    assert_evals_to!(r#"Num.toStr 10i32"#, RocStr::from("10"), RocStr);

    let max = format!("{}", i32::MAX);
    assert_evals_to!(
        r#"Num.toStr Num.maxI32"#,
        RocStr::from(max.as_str()),
        RocStr
    );

    let max = format!("{}", i32::MIN);
    assert_evals_to!(
        r#"Num.toStr Num.minI32"#,
        RocStr::from(max.as_str()),
        RocStr
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn num_to_str_i64() {
    use roc_std::RocStr;

    assert_evals_to!(r#"Num.toStr -10i64"#, RocStr::from("-10"), RocStr);
    assert_evals_to!(r#"Num.toStr -1i64"#, RocStr::from("-1"), RocStr);
    assert_evals_to!(r#"Num.toStr 0i64"#, RocStr::from("0"), RocStr);
    assert_evals_to!(r#"Num.toStr 1i64"#, RocStr::from("1"), RocStr);
    assert_evals_to!(r#"Num.toStr 10i64"#, RocStr::from("10"), RocStr);

    let max = format!("{}", i64::MAX);
    assert_evals_to!(
        r#"Num.toStr Num.maxI64"#,
        RocStr::from(max.as_str()),
        RocStr
    );

    let max = format!("{}", i64::MIN);
    assert_evals_to!(
        r#"Num.toStr Num.minI64"#,
        RocStr::from(max.as_str()),
        RocStr
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn u8_addition_greater_than_i8() {
    assert_evals_to!(
        indoc!(
            r#"
            x : U8
            x = 100
            y : U8
            y = 100
            x + y
            "#
        ),
        200,
        u8
    )
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn u8_sub_greater_than_i8() {
    assert_evals_to!(
        indoc!(
            r#"
            x : U8
            x = 255
            y : U8
            y = 55
            x - y
            "#
        ),
        200,
        u8
    )
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn u8_mul_greater_than_i8() {
    assert_evals_to!(
        indoc!(
            r#"
            x : U8
            x = 40
            y : U8
            y = 5
            x * y
            "#
        ),
        200,
        u8
    )
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn add_saturated() {
    assert_evals_to!(
        indoc!(
            r#"
            x : U8
            x = 200
            y : U8
            y = 200
            Num.addSaturated x y
            "#
        ),
        255,
        u8
    );

    assert_evals_to!(
        indoc!(
            r#"
            x : I8
            x = 100
            y : I8
            y = 100
            Num.addSaturated x y
            "#
        ),
        127,
        i8
    );

    assert_evals_to!(
        indoc!(
            r#"
            x : I8
            x = -100
            y : I8
            y = -100
            Num.addSaturated x y
            "#
        ),
        -128,
        i8
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn sub_saturated() {
    assert_evals_to!(
        indoc!(
            r#"
            x : U8
            x = 10
            y : U8
            y = 20
            Num.subSaturated x y
            "#
        ),
        0,
        u8
    );
    assert_evals_to!(
        indoc!(
            r#"
            x : I8
            x = -100
            y : I8
            y = 100
            Num.subSaturated x y
            "#
        ),
        -128,
        i8
    );
    assert_evals_to!(
        indoc!(
            r#"
            x : I8
            x = 100
            y : I8
            y = -100
            Num.subSaturated x y
            "#
        ),
        127,
        i8
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn mul_saturated() {
    assert_evals_to!(
        indoc!(
            r#"
            x : U8
            x = 20
            y : U8
            y = 20
            Num.mulSaturated x y
            "#
        ),
        255,
        u8
    );
    assert_evals_to!(
        indoc!(
            r#"
            x : I8
            x = -20
            y : I8
            y = -20
            Num.mulSaturated x y
            "#
        ),
        127,
        i8
    );
    assert_evals_to!(
        indoc!(
            r#"
            x : I8
            x = 20
            y : I8
            y = -20
            Num.mulSaturated x y
            "#
        ),
        -128,
        i8
    );
    assert_evals_to!(
        indoc!(
            r#"
            x : I8
            x = -20
            y : I8
            y = 20
            Num.mulSaturated x y
            "#
        ),
        -128,
        i8
    );
    assert_evals_to!(
        indoc!(
            r#"
            x : I8
            x = 20
            y : I8
            y = 20
            Num.mulSaturated x y
            "#
        ),
        127,
        i8
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn monomorphized_ints() {
    assert_evals_to!(
        indoc!(
            r#"
            x = 100

            f : U8, U32 -> Nat
            f = \_, _ -> 18

            f x x
            "#
        ),
        18,
        usize
    )
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn monomorphized_floats() {
    assert_evals_to!(
        indoc!(
            r#"
            x = 100.0

            f : F32, F64 -> Nat
            f = \_, _ -> 18

            f x x
            "#
        ),
        18,
        usize
    )
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn monomorphized_ints_names_dont_conflict() {
    assert_evals_to!(
        indoc!(
            r#"
            f : U8 -> Nat
            f = \_ -> 9
            x =
                n = 100
                f n

            y =
                n = 100
                f n

            x + y
            "#
        ),
        18,
        usize
    )
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn monomorphized_ints_aliased() {
    assert_evals_to!(
        indoc!(
            r#"
            app "test" provides [main] to "./platform"

            main =
                y = 100
                w1 = y
                w2 = y

                f1 : U8, U32 -> U8
                f1 = \_, _ -> 1

                f2 : U32, U8 -> U8
                f2 = \_, _ -> 2

                f1 w1 w2 + f2 w1 w2
            "#
        ),
        3,
        u8
    )
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn to_float_f32() {
    assert_evals_to!(
        indoc!(
            r#"
            n : U8
            n = 100

            f : F32
            f = Num.toFrac n
            f
            "#
        ),
        100.,
        f32
    )
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn to_float_f64() {
    assert_evals_to!(
        indoc!(
            r#"
            n : U8
            n = 100

            f : F64
            f = Num.toFrac n
            f
            "#
        ),
        100.,
        f64
    )
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
// https://github.com/rtfeldman/roc/issues/2696
fn upcast_of_int_is_zext() {
    assert_evals_to!(
        indoc!(
            r#"
            Num.toU16 0b1000_0000u8
            "#
        ),
        128,
        u16
    )
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
// https://github.com/rtfeldman/roc/issues/2696
fn upcast_of_int_checked_is_zext() {
    assert_evals_to!(
        indoc!(
            r#"
            when Num.toU16Checked 0b1000_0000u8 is
                Ok 128u16 -> 1u8
                _ -> 0u8
            "#
        ),
        1,
        u8
    )
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn modulo_of_unsigned() {
    assert_evals_to!(
        indoc!(
            r#"
            0b1111_1111u8 % 64
            "#
        ),
        63,
        u8
    )
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn div_of_unsigned() {
    assert_evals_to!(
        indoc!(
            r#"
            0b1111_1111u8 // 2
            "#
        ),
        127,
        u8
    )
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn dec_float_suffix() {
    assert_evals_to!(
        indoc!(
            r#"
            123.0dec
            "#
        ),
        RocDec::from_str_to_i128_unsafe("123.0"),
        i128
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn dec_no_decimal() {
    assert_evals_to!(
        indoc!(
            r#"
            3dec
            "#
        ),
        RocDec::from_str_to_i128_unsafe("3.0"),
        i128
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn ceiling_to_u32() {
    assert_evals_to!(
        indoc!(
            r#"
            n : U32
            n = Num.ceiling 124.5
            n
            "#
        ),
        125,
        u32
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn floor_to_u32() {
    assert_evals_to!(
        indoc!(
            r#"
            n : U32
            n = Num.floor 124.5
            n
            "#
        ),
        124,
        u32
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn round_to_u32() {
    assert_evals_to!(
        indoc!(
            r#"
            n : U32
            n = Num.round 124.49
            n
            "#
        ),
        124,
        u32
    );
}