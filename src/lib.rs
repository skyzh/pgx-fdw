use std::{ffi::c_int, pin::Pin};

use pgx::pg_sys;

/// Your foreign data wrapper should implement this trait, and it will be automatically registered into the PostgreSQL
/// server.
pub trait PgxFdw {
    type Options;
    type Iter: PgxIterator;

    fn get_foreign_rel_size(
        root: *mut pg_sys::PlannerInfo,
        baserel: *mut pg_sys::RelOptInfo,
        foreigntableid: pg_sys::Oid,
    );

    fn get_foreign_paths(
        root: *mut pg_sys::PlannerInfo,
        baserel: *mut pg_sys::RelOptInfo,
        foreigntableid: pg_sys::Oid,
    );

    fn get_foreign_plan(
        root: *mut pg_sys::PlannerInfo,
        baserel: *mut pg_sys::RelOptInfo,
        foreigntableid: pg_sys::Oid,
        best_path: *mut pg_sys::ForeignPath,
        tlist: *mut pg_sys::List,
        scan_clauses: *mut pg_sys::List,
        outer_plan: *mut pg_sys::Plan,
    ) -> *mut pg_sys::ForeignScan;

    fn explain_foreign_scan(
        #[allow(unused_variables)] node: *mut pg_sys::ForeignScanState,
        #[allow(unused_variables)] es: *mut pg_sys::ExplainState,
    ) {
    }

    fn begin_foreign_scan(
        node: *mut pg_sys::ForeignScanState,
        eflags: c_int,
    ) -> Pin<Box<Self::Iter>>;
}

/// You should implement this trait for your iterator / iterator state. You can access the state of the foreign data
/// wrapper when scanning.
pub trait PgxIterator {
    fn next(&mut self) -> *mut pg_sys::TupleTableSlot;
}

macro_rules! define_pgx_fdw {
    () => {};
}
