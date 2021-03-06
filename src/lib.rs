//! An alternate implementation of `try!` that can operate on `Option<T>`
//! as well as `Result<T, E>`, while remaining backward compatible (I think).

pub trait Try<T, E> {
    fn try(self) -> Result<T, E>;
}

#[macro_export]
macro_rules! try2 {
    ( $e:expr ) => {
        match $crate::Try::try($e) {
            Ok(v) => v,
            Err(r) => return r
        }
    }
}

impl<T, R> Try<T, Option<R>> for Option<T> {
    fn try(self) -> Result<T, Option<R>> {
        match self {
            Some(v) => Ok(v),
            None => Err(None)
        }
    }
}

impl<T, E, R> Try<T, Result<T, R>> for Result<T, E>
        where R: From<E> {
    fn try(self) -> Result<T, Result<T, R>> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => Err(Err(::std::convert::From::from(e)))
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_option() {
        assert_eq!(try_option_none(), None);
        assert_eq!(try_option_some(), Some(1));
        assert_eq!(try_option_alt(), Some("1".to_string()));
    }

    fn try_option_none() -> Option<i32> {
        let a = try2!(None);
        Some(a)
    }

    fn try_option_some() -> Option<i32> {
        let a = try2!(Some(1));
        Some(a)
    }

    fn try_option_alt() -> Option<String> {
        let a = try2!(Some(1));
        Some(a.to_string())
    }

    #[test]
    fn test_result() {
        assert_eq!(try_result_ok(), Ok(1));
        assert_eq!(try_result_err(), Err("foo"));
        assert_eq!(try_result_alt(), Err("foo".to_string()));
    }

    fn try_result_ok() -> Result<i32, ()> {
        let v = try2!(Ok(1));
        Ok(v)
    }

    fn try_result_err() -> Result<(), &'static str> {
        let v = try2!(Err("foo"));
        Ok(v)
    }

    fn try_result_alt() -> Result<(), String> {
        let v = try2!(try_result_err());
        Ok(v)
    }
}
