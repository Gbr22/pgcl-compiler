use super::tree::ParseError;
use crate::common::range::Range;
use rayon::{iter::Either, prelude::*, vec::IntoIter};

#[derive(Debug, Clone)]
pub struct PtError {
    pub range: Option<Range>,
    pub message: String,
}

impl From<ParseError> for PtError {
    fn from(value: ParseError) -> Self {
        PtError {
            range: Some(value.range),
            message: value.text,
        }
    }
}

pub fn try_map_into<Source, Destination, MyErr>(vec: Vec<Source>) -> Result<Vec<Destination>, MyErr>
where
    Destination: Sized + Sync + Send,
    MyErr: Sized + Sync + Send + Clone,
    Source: TryInto<Destination, Error = MyErr> + Sized + Sync + Send,
{
    let iter: IntoIter<Source> = vec.into_par_iter();
    let vecs: (Vec<Destination>, Vec<MyErr>) = iter.partition_map(|fun: Source| {
        let result: Result<Destination, MyErr> = fun.try_into();

        match result {
            Ok(ok) => Either::Left(ok),
            Err(err) => Either::Right(err),
        }
    });

    if let Some(err) = vecs.1.first() {
        return Err(err.clone());
    };

    Ok(vecs.0)
}
