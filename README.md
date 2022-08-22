# Decimal WAD
  
Math for preserving precision floats which are limited to be at most u64::MAX.

- Decimals are internally scaled by a WAD (10^18) to preserve precision up to 18 decimal places.
- Decimals are sized to support both serialization and precise math for the full range of unsigned 64-bit integers.
- The underlying representation of decimals is a u192 rather than u256 to reduce compute cost while losing support for arithmetic operations at the high end of u64 range.
- Rates are sized to support both serialization and precise math for the full range of unsigned 8-bit integers.
- The underlying representation of rates is a u128 rather than u192 to reduce compute cost while losing support for arithmetic operations at the high end of u8 range.
