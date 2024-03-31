pub fn create_paginator(table_name: &str) -> impl Fn(&str, &str, &str, &str, u32, u32) -> String {
    let table_name = table_name.to_string(); // Clone table_name
    let paginate = move |columns: &str,
                         q: &str,
                         sort_by: &str,
                         sort_direction: &str,
                         limit: u32,
                         offset: u32| {
        let mut result = String::from("SELECT ");
        result.push_str(columns);

        result.push_str(format!(" FROM {}", table_name).as_str());

        if !q.is_empty() {
            result.push_str(format!(" WHERE {}", q).as_str());
        }

        if !sort_by.is_empty() {
            result.push_str(format!(" ORDER BY {}", sort_by).as_str());
            if !sort_direction.is_empty() {
                result.push_str(format!(" {}", sort_direction).as_str());
            } else {
                result.push_str(" ASC");
            }
        }

        if limit > 0 {
            result.push_str(format!(" LIMIT {}", limit).as_str());
        }

        if offset > 0 {
            result.push_str(format!(" OFFSET {}", offset.to_string().as_str()).as_str());
        }

        result.push(';');
        result
    };
    paginate
}
