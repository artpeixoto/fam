

#![allow(unused_parens)]

#![deny(
	unconditional_recursion,
	unused_must_use,
)]

#![feature(
	const_type_name,
	adt_const_params,
	slice_as_array,
	bigint_helper_methods,
	strict_overflow_ops,
	mixed_integer_ops_unsigned_sub,
	unique_rc_arc,
	let_chains,
	coroutines,
	coroutine_trait,
	stmt_expr_attributes
)]


extern crate core;

pub mod word;
pub mod application;
pub mod tools;

pub const PROGRAM_COUNTER_REGISTER_ADDR: usize = 63;

pub type Step = u32;


pub trait Draw{
	fn draw(&self);
}