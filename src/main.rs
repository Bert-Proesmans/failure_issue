extern crate failure;
#[macro_use]
extern crate failure_derive;

mod error {
	use std::fmt::{Display, Debug};
	
    #[derive(Debug, Fail)]
    #[fail(display = "The entity with id `{:}`was not found", _0)]
    pub struct MissingEntityError<ID>
    where
        ID: Display + Debug,
    {
        pub id: ID,
    }
}

use failure::{Fail, Error};
use self::error::MissingEntityError;

fn main() {
    let id: usize = 0;
    let error = MissingEntityError { id };
    let fail: &Fail = &MissingEntityError { id };
    let err: Error = MissingEntityError { id }.into();
}
