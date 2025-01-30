use crate::ansi;

#[derive(Clone)]
pub struct Table {
    rows: Vec<Row>,
    format: TableFormat,
}

#[derive(Clone)]
pub struct Row {
    cells: Vec<Cell>,
}

#[derive(Clone)]
pub struct Cell {
    content: String,
    padding: (usize, usize),
}

#[derive(Clone)]
pub struct TableFormat {
    column_separator: &'static str,
    row_separator: &'static str,
    borders: Borders,
}

#[derive(Clone)]
pub struct Borders {
    internal_h: bool,
}

pub mod format {
    use super::*;

    pub mod consts {
        use super::*;

        pub static FORMAT_CLEAN: &TableFormat = &TableFormat {
            column_separator: "",
            row_separator: "",
            borders: Borders { internal_h: false },
        };
    }
}
#[macro_export]
macro_rules! row {
    ($($x:expr),* $(,)?) => {
        $crate::table::Row::new(vec![
            $(
                $crate::table::Cell::new($x)
            ),*
        ])
    };
}

impl Table {
    pub fn new() -> Self {
        Table {
            rows: Vec::new(),
            format: format::consts::FORMAT_CLEAN.clone(),
        }
    }

    pub fn set_format(&mut self, format: &TableFormat) {
        self.format = format.clone();
    }

    pub fn add_row(&mut self, row: Row) {
        self.rows.push(row);
    }

    pub fn print<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        let column_widths = self.get_column_widths();
        for (i, row) in self.rows.iter().enumerate() {
            self.print_row(writer, row, &column_widths)?;

            if self.format.borders.internal_h && i < self.rows.len() - 1 {
                self.print_separator(writer, &column_widths)?;
            }
        }

        Ok(())
    }

    fn print_separator<W: std::io::Write>(
        &self,
        writer: &mut W,
        widths: &[usize],
    ) -> std::io::Result<()> {
        if !self.format.row_separator.is_empty() {
            for (i, width) in widths.iter().enumerate() {
                if i > 0 {
                    write!(writer, "{}", self.format.column_separator)?;
                }
                write!(writer, "{}", self.format.row_separator.repeat(*width))?;
            }
            writeln!(writer)?;
        }
        Ok(())
    }

    fn get_column_widths(&self) -> Vec<usize> {
        let column_count = self
            .rows
            .iter()
            .map(|row| row.cells.len())
            .max()
            .unwrap_or(0);

        let mut widths = vec![0; column_count];

        for row in &self.rows {
            for (i, cell) in row.cells.iter().enumerate() {
                let max_line_width = cell
                    .content
                    .lines()
                    .map(strip_ansi_width)
                    .max()
                    .unwrap_or(0);
                let cell_width = max_line_width + cell.padding.0 + cell.padding.1;
                if i < widths.len() {
                    widths[i] = widths[i].max(cell_width);
                }
            }
        }

        widths
    }

    fn print_row<W: std::io::Write>(
        &self,
        writer: &mut W,
        row: &Row,
        widths: &[usize],
    ) -> std::io::Result<()> {
        let cell_lines: Vec<Vec<&str>> = row
            .cells
            .iter()
            .map(|cell| cell.content.lines().collect())
            .collect();

        let max_lines = cell_lines
            .iter()
            .map(|lines| lines.len())
            .max()
            .unwrap_or(0);

        for line_idx in 0..max_lines {
            for (cell_idx, cell) in row.cells.iter().enumerate() {
                let content_line = cell_lines[cell_idx].get(line_idx).unwrap_or(&"");

                let visible_len = strip_ansi_width(content_line);
                let left_pad = cell.padding.0;
                let right_pad = cell.padding.1;
                let padded_visible = visible_len + left_pad + right_pad;

                let column_width = *widths.get(cell_idx).unwrap_or(&0);
                let available_space = column_width as i32 - padded_visible as i32;

                // Simple always-left alignment:
                let align_left = 0;
                let align_right = if available_space > 0 {
                    available_space as usize
                } else {
                    0
                };

                let mut line = String::new();
                line.push_str(&" ".repeat(align_left));
                line.push_str(&" ".repeat(left_pad));
                line.push_str(content_line);
                line.push_str(&" ".repeat(right_pad));
                line.push_str(&" ".repeat(align_right));

                let final_line = ansi::truncate(&line, column_width);
                write!(writer, "{}", final_line)?;

                if cell_idx < row.cells.len() - 1 {
                    write!(writer, "{}", self.format.column_separator)?;
                }
            }
            writeln!(writer)?;
        }
        Ok(())
    }
}

impl Row {
    pub fn new(cells: Vec<Cell>) -> Self {
        Row { cells }
    }
}

impl Cell {
    pub fn new<S: Into<String>>(content: S) -> Self {
        Cell {
            content: content.into(),
            padding: (1, 1),
        }
    }
}

fn strip_ansi_width(s: &str) -> usize {
    let mut visible_len = 0;
    let mut in_escape = false;

    for c in s.chars() {
        if c == '\x1B' {
            in_escape = true;
            continue;
        }
        if in_escape {
            if c == 'm' {
                in_escape = false;
            }
            continue;
        }
        visible_len += 1;
    }
    visible_len
}
