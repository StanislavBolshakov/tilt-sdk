use comfy_table::Table;

use crate::output::rows::ToStringRow;

pub fn format_table<T: ToStringRow>(rows: &[T]) -> String {
    if rows.is_empty() {
        return String::new();
    }
    let mut table = Table::new();
    table.set_style(comfy_table::TableComponent::VerticalLines, ' ');

    let headers = rows[0].to_headers();
    table.set_header(headers);

    for row in rows {
        table.add_row(row.to_row());
    }

    table.to_string()
}
