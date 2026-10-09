mod add_column_not_null;
mod create_index_concurrently;

use crate::rule::Rule;

pub fn all() -> Vec<Box<dyn Rule>> {
    vec![
        Box::new(create_index_concurrently::CreateIndexConcurrently),
        Box::new(add_column_not_null::AddColumnNotNull),
    ]
}
