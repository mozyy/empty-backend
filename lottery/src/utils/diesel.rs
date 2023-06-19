use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::query_builder::*;
use diesel::query_dsl::methods::LoadQuery;
use diesel::sql_types::BigInt;

use crate::pb;

pub trait Paginate: Sized {
    fn paginate(self, paginate: Option<pb::paginate::Paginate>) -> Paginated<Self>;
}

impl<T> Paginate for T {
    fn paginate(self, paginate: Option<pb::paginate::Paginate>) -> Paginated<Self> {
        let paginate = paginate.unwrap_or(pb::paginate::Paginate {
            page: 1,
            per_page: 10,
        });
        let offset = (paginate.page - 1) * paginate.per_page;
        Paginated {
            query: self,
            paginate,
            offset,
        }
    }
}

#[derive(Debug, Clone, QueryId)]
pub struct Paginated<T> {
    query: T,
    paginate: pb::paginate::Paginate,
    offset: i64,
}

impl<T> Paginated<T> {
    pub fn load_and_paginated<'a, U>(
        self,
        conn: &mut PgConnection,
    ) -> QueryResult<(Vec<U>, Option<pb::paginate::Paginated>)>
    where
        Self: LoadQuery<'a, PgConnection, (U, i64)>,
    {
        let pb::paginate::Paginate { page, per_page } = self.paginate;
        let results = self.load::<(U, i64)>(conn)?;
        let total = results.get(0).map(|x| x.1).unwrap_or(0);
        let records = results.into_iter().map(|x| x.0).collect();
        let total_pages = (total as f64 / per_page as f64).ceil() as i64;
        Ok((
            records,
            Some(pb::paginate::Paginated {
                page,
                per_page,
                total,
                total_pages,
            }),
        ))
    }
}

impl<T: Query> Query for Paginated<T> {
    type SqlType = (T::SqlType, BigInt);
}

impl<T> RunQueryDsl<PgConnection> for Paginated<T> {}

impl<T> QueryFragment<Pg> for Paginated<T>
where
    T: QueryFragment<Pg>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, Pg>) -> QueryResult<()> {
        out.push_sql("SELECT *, COUNT(*) OVER () FROM (");
        self.query.walk_ast(out.reborrow())?;
        out.push_sql(") t LIMIT ");
        out.push_bind_param::<BigInt, _>(&self.paginate.per_page)?;
        out.push_sql(" OFFSET ");
        out.push_bind_param::<BigInt, _>(&self.offset)?;
        Ok(())
    }
}
