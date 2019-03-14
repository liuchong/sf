use crate::id::Id;
use diesel::backend::Backend;
use diesel::deserialize::FromSql;
use diesel::expression::AsExpression;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::sql_types::{BigInt, Nullable};
use std::error::Error;
use std::io::Write;

type DieselError = Box<Error + Send + Sync>;

#[derive(FromSqlRow, SqlType)]
#[diesel(foreign_derive)]
#[allow(dead_code)]
struct Proxy(Id);

impl AsExpression<BigInt> for Id {
    type Expression = <i64 as AsExpression<BigInt>>::Expression;
    fn as_expression(self) -> Self::Expression {
        <i64 as AsExpression<BigInt>>::as_expression(Into::<i64>::into(self))
    }
}

impl<'a> AsExpression<BigInt> for &'a Id {
    type Expression = <i64 as AsExpression<BigInt>>::Expression;
    fn as_expression(self) -> Self::Expression {
        <i64 as AsExpression<BigInt>>::as_expression(Into::<i64>::into(self))
    }
}

impl AsExpression<Nullable<BigInt>> for Id {
    type Expression = <i64 as AsExpression<Nullable<BigInt>>>::Expression;
    fn as_expression(self) -> Self::Expression {
        <i64 as AsExpression<Nullable<BigInt>>>::as_expression(
            Into::<i64>::into(self),
        )
    }
}

impl<'a> AsExpression<Nullable<BigInt>> for &'a Id {
    type Expression = <i64 as AsExpression<Nullable<BigInt>>>::Expression;
    fn as_expression(self) -> Self::Expression {
        <i64 as AsExpression<Nullable<BigInt>>>::as_expression(
            Into::<i64>::into(self),
        )
    }
}

impl<DB: Backend<RawValue = [u8]>> FromSql<BigInt, DB> for Id {
    fn from_sql(bytes: Option<&[u8]>) -> Result<Self, DieselError> {
        <i64 as FromSql<_, DB>>::from_sql(bytes)
            .and_then(|i| Ok(Into::<Id>::into(i)))
    }
}

impl<DB: Backend> ToSql<BigInt, DB> for Id {
    fn to_sql<W: Write>(
        &self,
        out: &mut Output<W, DB>,
    ) -> Result<IsNull, DieselError> {
        let i: i64 = self.into();
        <i64 as ToSql<BigInt, DB>>::to_sql(&i, out)
    }
}
