use crate::ansi;

#[derive(Clone)]
pub struct Table {
    rows: Vec<Row>,
    format: TableFormat,
    width: Option<usize>,
}

#[derive(Clone)]
pub struct Row {
    cells: Vec<Cell>,
}

#[derive(Clone)]
pub struct Cell {
    content: String,
    align: Alignment,
    padding: (usize, usize),
}

#[derive(Clone)]
pub struct TableFormat {
    column_separator: &'static str,
    row_separator: &'static str,
    padding: usize,
    borders: Borders,
}

#[derive(Clone)]
pub struct Borders {
    top: bool,
    bottom: bool,
    left: bool,
    right: bool,
    internal_h: bool,
    internal_v: bool,
}

#[derive(Clone, Copy)]
pub enum Alignment {
    Left,
    Center,
    Right,
}

// Constants using static str instead of String
pub mod format {
    use super::*;

    pub mod consts {
        use super::*;

        pub static FORMAT_CLEAN: &TableFormat = &TableFormat {
            column_separator: "",
            row_separator: "",
            padding: 1,
            borders: Borders {
                top: false,
                bottom: false,
                left: false,
                right: false,
                internal_h: false,
                internal_v: false,
            },
        };

        pub static FORMAT_BORDERS: &TableFormat = &TableFormat {
            column_separator: "|",
            row_separator: "-",
            padding: 1,
            borders: Borders {
                top: true,
                bottom: true,
                left: true,
                right: true,
                internal_h: true,
                internal_v: true,
            },
        };
    }
}

// Export the macro at the crate root
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
            width: None,
        }
    }

    pub fn set_format(&mut self, format: &TableFormat) {
        self.format = format.clone();
    }

    pub fn set_width(&mut self, width: Option<usize>) {
        self.width = width;
    }

    pub fn add_row(&mut self, row: Row) {
        self.rows.push(row);
    }

    pub fn print<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        let column_widths = self.get_column_widths();

        // Print all rows with proper row separation
        for (i, row) in self.rows.iter().enumerate() {
            // Print the current row
            self.print_row(writer, row, &column_widths)?;

            // Add row separator if needed and not the last row
            if self.format.borders.internal_h && i < self.rows.len() - 1 {
                // Add row separator line here if needed
                self.print_separator(writer, &column_widths)?;
            }
        }

        Ok(())
    }

    // Helper method to print row separators if needed
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

        // Adjust for terminal width if set
        if let Some(term_width) = self.width {
            let total_width: usize = widths.iter().sum();
            let separator_width = self.format.column_separator.len() * (widths.len() - 1);

            let available_width = term_width.saturating_sub(separator_width);

            if total_width > available_width {
                let ratio = available_width as f64 / total_width as f64;
                for width in &mut widths {
                    *width = (*width as f64 * ratio).floor() as usize;
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

                let (align_left, align_right) = if available_space >= 0 {
                    match cell.align {
                        Alignment::Left => (0, available_space as usize),
                        Alignment::Right => (available_space as usize, 0),
                        Alignment::Center => {
                            let left = (available_space as usize) / 2;
                            (left, available_space as usize - left)
                        }
                    }
                } else {
                    (0, 0)
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
            align: Alignment::Left,
            padding: (1, 1),
        }
    }

    pub fn align(mut self, align: Alignment) -> Self {
        self.align = align;
        self
    }

    pub fn padding(mut self, left: usize, right: usize) -> Self {
        self.padding = (left, right);
        self
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

// Terminal width detection
pub fn get_terminal_width() -> Option<usize> {
    termsize::get().map(|size| size.cols as usize)
}
