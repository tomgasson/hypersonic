use std::{path::PathBuf, env, borrow::Cow};

use parcel_node_resolver::{CacheCow, OsFileSystem, SpecifierType, Resolution, Resolver, Cache};

pub fn resolve(
    specifier: &str,
    from: &PathBuf,
) -> Result<PathBuf, String> {
    let resolver = Resolver::node(
        Cow::Owned(env::current_dir().unwrap().as_path().into()), 
        CacheCow::Owned(Cache::new(OsFileSystem)),
    );

    let resolve_result = resolver.resolve(specifier, from, SpecifierType::Esm);

    if resolve_result.result.is_err() {
        let msg = format!("Resolve Result {:?}", resolve_result.result.err());
        return Err(msg);
    }

    let (resolution, _) = resolve_result.result.unwrap();

    return match resolution {
        Resolution::Path(p) => Ok(p),
        Resolution::Builtin(b) => {
            let msg = format!("Resolution::Builtin {:?}", b);
            return Err(msg);
        },
        Resolution::External => {
            let msg = format!("Resolution::External");
            return Err(msg);
        },
        Resolution::Empty => {
            let msg = format!("Resolution::Empty");
            return Err(msg);
        },
        Resolution::Global(s) => {
            let msg = format!("Resolution::Global {:?}", s);
            return Err(msg);
        },
    }
}
