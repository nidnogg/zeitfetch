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
    fn print_separator<W: std::io::Write>(&self, writer: &mut W, widths: &[usize]) -> std::io::Result<()> {
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
        let column_count = self.rows.iter()
            .map(|row| row.cells.len())
            .max()
            .unwrap_or(0);
        
        let mut widths = vec![0; column_count];
        
        for row in &self.rows {
            for (i, cell) in row.cells.iter().enumerate() {
                let visible_width = strip_ansi_width(&cell.content);
                widths[i] = widths[i].max(visible_width + cell.padding.0 + cell.padding.1);
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

    fn print_row<W: std::io::Write>(&self, writer: &mut W, row: &Row, widths: &[usize]) -> std::io::Result<()> {
        println!("DEBUG: Starting print_row");
        println!("DEBUG: Number of cells: {}", row.cells.len());
        
        // Print raw content of each cell
        for (i, cell) in row.cells.iter().enumerate() {
            println!("DEBUG: Cell {} raw content:\n{}", i, cell.content);
            println!("DEBUG: Cell {} content bytes: {:?}", i, cell.content.as_bytes());
        }
        
        // Split each cell's content into lines
        let cell_lines: Vec<Vec<&str>> = row.cells.iter()
            .map(|cell| {
                let lines = cell.content.lines().collect::<Vec<_>>();
                println!("DEBUG: Cell split into {} lines", lines.len());
                for (i, line) in lines.iter().enumerate() {
                    println!("DEBUG: Line {}: '{}'", i, line);
                }
                lines
            })
            .collect();
        
        let max_lines = cell_lines.iter()
            .map(|lines| lines.len())
            .max()
            .unwrap_or(0);
        
        println!("DEBUG: Max lines: {}", max_lines);
        println!("DEBUG: Column widths: {:?}", widths);
        
        // Print each line
        for line_idx in 0..max_lines {
            for (cell_idx, lines) in cell_lines.iter().enumerate() {
                if cell_idx > 0 {
                    write!(writer, "         ")?;
                }
                
                let content = lines.get(line_idx).unwrap_or(&"");
                println!("DEBUG: Writing line {} for cell {}: '{}'", line_idx, cell_idx, content);
                write!(writer, "{}", content)?;
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