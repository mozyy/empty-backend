use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::query_builder::*;
use diesel::query_dsl::methods::LoadQuery;
use diesel::sql_types::BigInt;
use diesel::sql_types::Integer;

use crate::pb;

pub trait Paginate: Sized {
    fn paginate(self, paginate: Option<pb::utils::paginate::Paginate>) -> Paginated<Self>;
}

impl<T> Paginate for T {
    fn paginate(self, paginate: Option<pb::utils::paginate::Paginate>) -> Paginated<Self> {
        let offset = if let Some(paginate) = &paginate {
            (paginate.page - 1) * paginate.per_page
        } else {
            0
        };
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
    paginate: Option<pb::utils::paginate::Paginate>,
    offset: i32,
}

impl<T> Paginated<T> {
    pub fn load_and_paginated<'a, U>(
        self,
        conn: &mut PgConnection,
    ) -> QueryResult<(Vec<U>, Option<pb::utils::paginate::Paginated>)>
    where
        Self: LoadQuery<'a, PgConnection, (U, i64)>,
    {
        if let Some(paginate) = &self.paginate {
            let pb::utils::paginate::Paginate { page, per_page } = paginate.clone();
            let results = self.load::<(U, i64)>(conn)?;
            let total = results.get(0).map(|x| x.1).unwrap_or(0);
            let records = results.into_iter().map(|x| x.0).collect();
            let total_pages = (total as f64 / per_page as f64).ceil() as i32;
            let paginated = Some(pb::utils::paginate::Paginated {
                page,
                per_page,
                total: total as i32,
                total_pages,
            });
            Ok((records, paginated))
        } else {
            let results = self.load::<(U, i64)>(conn)?;
            let records = results.into_iter().map(|x| x.0).collect();
            Ok((records, None))
        }
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
        if let Some(paginate) = &self.paginate {
            out.push_sql("SELECT *, COUNT(*) OVER () FROM (");
            self.query.walk_ast(out.reborrow())?;
            out.push_sql(") t LIMIT ");
            out.push_bind_param::<Integer, _>(&paginate.per_page)?;
            out.push_sql(" OFFSET ");
            out.push_bind_param::<Integer, _>(&self.offset)?;
        } else {
            out.push_sql("SELECT *, 0::bigint FROM (");
            self.query.walk_ast(out.reborrow())?;
            out.push_sql(") t");
        }
        Ok(())
    }
}
