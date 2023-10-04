//!
//!

pub const Token = union(enum) {
  identifier: []u8,
  integer: isize,
  string: []u8,
  let
};
