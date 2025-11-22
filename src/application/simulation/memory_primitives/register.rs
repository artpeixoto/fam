use std::ops::Deref;
use std::sync::{Arc, RwLock, RwLockWriteGuard};
use crate::{Step};


pub struct Register<Data>{
	value		: Data,
}


impl<Data> Register<Data>{
	pub fn new(initial_value: Data) -> Self{
		Self{
			value		: initial_value,
		}
	}
	pub fn write(&mut self, new_value: Data){
		self.value		= new_value;
	}
	pub fn read<'a>(&'a self) -> impl Deref<Target=Data> + 'a{
		&self.value
	}
}
