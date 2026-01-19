use diesel::QueryResult;
use diesel::backend::Backend;
use diesel::query_builder::{AstPass, QueryFragment};
use diesel::result::Error;
use diesel_cte_ext::{Columns, RecursiveParts};
use std::collections::BTreeSet;

#[derive(Debug, Clone)]
pub struct WithRecursiveNoUnion<DB: Backend, Cols, Seed, Step, Body> {
    pub(crate) cte_name: &'static str,
    pub(crate) columns: Columns<Cols>,
    pub(crate) seed: Seed,
    pub(crate) step: Step,
    pub(crate) body: Body,
    pub(crate) _marker: std::marker::PhantomData<DB>,
}

impl<DB, Cols, Seed, Step, Body> QueryFragment<DB>
    for WithRecursiveNoUnion<DB, Cols, Seed, Step, Body>
where
    DB: Backend,
    Seed: QueryFragment<DB>,
    Step: QueryFragment<DB>,
    Body: QueryFragment<DB>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        out.push_sql("WITH RECURSIVE ");
        out.push_identifier(self.cte_name)?;
        push_identifiers(&mut out, &self.columns)?;
        out.push_sql(" AS (");
        self.seed.walk_ast(out.reborrow())?;
        out.push_sql(" UNION ");
        self.step.walk_ast(out.reborrow())?;
        out.push_sql(") ");
        self.body.walk_ast(out.reborrow())
    }
}

/// Build a recursive CTE query.
pub fn with_recursive<DB, Cols, Seed, Step, Body, ColSpec>(
    cte_name: &'static str,
    columns: ColSpec,
    parts: RecursiveParts<Seed, Step, Body>,
) -> WithRecursiveNoUnion<DB, Cols, Seed, Step, Body>
where
    DB: RecursiveBackend,
    Seed: QueryFragment<DB>,
    Step: QueryFragment<DB>,
    Body: QueryFragment<DB>,
    ColSpec: Into<Columns<Cols>>,
{
    WithRecursiveNoUnion {
        cte_name,
        columns: columns.into(),
        seed: parts.seed,
        step: parts.step,
        body: parts.body,
        _marker: std::marker::PhantomData,
    }
}

fn push_identifiers<DB, Cols>(
    out: &mut AstPass<'_, '_, DB>,
    cols: &Columns<Cols>,
) -> QueryResult<()>
where
    DB: Backend,
{
    let ids = cols.names;
    if ids.is_empty() {
        return Ok(());
    }
    ensure_unique_columns(ids)?;
    out.push_sql(" (");
    for (i, id) in ids.iter().enumerate() {
        if i > 0 {
            out.push_sql(", ");
        }
        out.push_identifier(id)?;
    }
    out.push_sql(")");
    Ok(())
}

fn ensure_unique_columns(names: &[&str]) -> QueryResult<()> {
    let mut seen = BTreeSet::new();
    for name in names {
        if !seen.insert(name) {
            return Err(Error::QueryBuilderError(
                format!("duplicate column name '{name}' in CTE").into(),
            ));
        }
    }
    Ok(())
}

pub trait RecursiveBackend: Backend {}

#[cfg(feature = "sqlite")]
impl RecursiveBackend for diesel::sqlite::Sqlite {}

#[cfg(feature = "postgres")]
impl RecursiveBackend for diesel::pg::Pg {}

#[cfg(test)]
mod tests {
    use super::*;
    use diesel::debug_query;
    use diesel::dsl::sql;
    use diesel::sql_types::Integer;
    use diesel::sqlite::Sqlite;

    impl RecursiveBackend for diesel::sqlite::Sqlite {}

    #[test]
    fn recursive_builder_composes_fragments() {
        let query = with_recursive::<Sqlite, _, _, _, _, _>(
            "nums",
            &["n"],
            RecursiveParts::new(
                sql::<Integer>("SELECT 1"),
                sql::<Integer>("SELECT n + 1 FROM nums"),
                sql::<Integer>("SELECT n FROM nums"),
            ),
        );
        let sql = normalise_debug_sql(&debug_query::<Sqlite, _>(&query).to_string());
        assert_eq!(
            sql,
            "WITH RECURSIVE \"nums\" (\"n\") AS (SELECT 1 UNION SELECT n + 1 FROM nums) SELECT n FROM nums"
        );
    }

    fn normalise_debug_sql(sql: &str) -> String {
        let trimmed = sql.trim();
        let without_binds = trimmed
            .split_once(" -- binds: ")
            .map_or(trimmed, |(statement, _)| statement)
            .trim_end();
        without_binds.replace('`', "\"")
    }
}
