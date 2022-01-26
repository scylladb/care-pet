use once_cell::sync::Lazy;

pub(super) static KEYSPACE: &str = include_str!("keyspace.cql");

pub(super) static MIGRATE: Lazy<Vec<String>> = Lazy::new(|| {
    include_str!("migrate.cql")
        .split(';')
        .filter_map(|s| {
            let s = s.trim();
            if s.is_empty() {
                None
            } else {
                Some(s.to_owned())
            }
        })
        .collect()
});
