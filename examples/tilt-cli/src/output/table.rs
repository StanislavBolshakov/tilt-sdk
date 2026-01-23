use comfy_table::Cell;
use comfy_table::Table;

pub trait ToStringRow {
    fn to_headers(&self) -> Vec<String>;
    fn to_row(&self) -> Vec<Cell>;
}

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
