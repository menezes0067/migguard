mod create_index_concurrently;
mod add_column_not_null;

use crate::rule::Rule;

pub fn all() -> Vec<Box<dyn Rule>> {
    vec![
        Box::new(create_index_concurrently::CreateIndexConcurrently),
    ]
}