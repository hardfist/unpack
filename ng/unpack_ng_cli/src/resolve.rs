pub mod pattern;
pub mod parse;
use turbo_rcstr::RcStr;
use turbo_tasks::ResolvedVc;

use crate::resolve::pattern::Pattern;

#[turbo_tasks::value(shared)]
#[derive(Hash, Clone, Debug)]
pub enum Request {
    Raw {
        path: Pattern,
        query: RcStr,
        force_in_lookup_dir: bool,
        fragment: RcStr,
    },
    Relative {
        path: Pattern,
        query: RcStr,
        force_in_lookup_dir: bool,
        fragment: RcStr,
    },
    Module {
        module: RcStr,
        path: Pattern,
        query: RcStr,
        fragment: RcStr,
    },
    ServerRelative {
        path: Pattern,
        query: RcStr,
        fragment: RcStr,
    },
    Windows {
        path: Pattern,
        query: RcStr,
        fragment: RcStr,
    },
    Empty,
    PackageInternal {
        path: Pattern,
    },
    Uri {
        protocol: RcStr,
        remainder: RcStr,
        query: RcStr,
        fragment: RcStr,
    },
    DataUri {
        media_type: RcStr,
        encoding: RcStr,
        data: ResolvedVc<RcStr>,
    },
    Unknown {
        path: Pattern,
    },
    Dynamic,
    Alternatives {
        requests: Vec<ResolvedVc<Request>>,
    },
}