// use std::ops::Deref;

use std::fmt::{self, Display, Formatter};
use std::result::Result;
use thiserror::Error;

use crate::Document;

#[derive(Debug, Error, PartialEq)]
#[non_exhaustive]
pub enum TableError {
    #[error("Wrong number of cells provided. Provided {0} cells, require {1} columns")]
    WrongNumberOfColumns(usize, usize),
}
/// A cell in a table
#[derive(Clone, Debug, PartialEq)]
pub struct Cell {
    /// content of the cell
    /// for the time being this is a String, it should be a paragraph without newline
    pub value: String,
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", &self.value)
    }
}
// impl Deref for Cell {
//     type Target = str;
//     fn deref(&self) -> &self::Target {
//         &self.0
//     }
// }

/// A row in a table
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Row {
    /// content of the row
    pub cells: Vec<Cell>,
    /// is it a header row
    pub is_header: bool,
    /// is this the first header
    pub is_first_header: bool,
}

impl Row {
    /// create a Row
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Add a cell to the row
    /// needs better implementation. Some Latex Element, not string
    pub fn push_cell(&mut self, content: String) {
        let cell = Cell { value: content };
        self.cells.push(cell);
    }
}

impl Display for Row {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut row = Vec::new();
        for cell in &self.cells {
            row.push((&cell.value).to_string());
        }
        // Todo: There must be a better way to do this
        let mut temp = row.join(" & ");
        temp.push_str(" \\\\");
        write!(f, "{}", temp)
    }
}

/// the kind of table
#[derive(Clone, Debug, PartialEq)]
pub enum TableKind {
    /// A standard table
    /// does not (yet) include the optional table container
    Tabular,
    /// A tabularx table
    Tabularx,
    /// A longtable
    LongTable,
    /// A long tabularx
    XLTabular,
}

impl TableKind {
    /// Get the TableKind environment names
    pub fn environment_name(&self) -> &str {
        match *self {
            TableKind::Tabular => "tabular",
            TableKind::Tabularx => "tabularx",
            TableKind::LongTable => "longtable",
            TableKind::XLTabular => "xltabular",
        }
    }
}

/// A table of various kind
#[derive(Clone, Debug, PartialEq)]
pub struct Table {
    /// The kind of table
    pub kind: TableKind,
    /// All rows
    pub rows: Vec<Row>,
    /// Width of the table for tabularx and its derivatives
    pub table_width: String,

    column_count: usize,
    /// Column types as String. i.e. llXrr
    pub column_types: String,
}

impl Table {
    /// Create an empty table of the specified type with the specified column types
    pub fn new(kind: TableKind, table_width: String, column_types: String) -> Table {
        Table {
            kind,
            rows: Vec::new(),
            table_width,
            column_count: column_types.chars().count(),
            column_types,
        }
    }

    /// Add a row to the table and counts the the number of [`crate::Cell`] pushed.
    ///
    /// # Note
    /// If the number of [`Cell`]s in the [`Row`] do not match the number of columns configured
    /// in the [`Table`], a [`TableError`] is generated.
    pub fn push_row(&mut self, row: Row) -> Result<&mut Table, TableError> {
        let provided_cells = row.cells.iter().count();
        if provided_cells == self.column_count() {
            self.rows.push(row);
            Ok(self)
        } else {
            Err(TableError::WrongNumberOfColumns(
                provided_cells,
                self.column_count(),
            ))
        }
    }

    /// Number of columns in the table.
    ///
    /// This value is derived from the column types provided to the table constructor. It assumes only ASCII characters as column types.
    ///
    /// # Example
    ///
    /// \\begin{tabularx}{\textwidth}{llXrr} would result in five columns. Column count is 5.
    pub fn column_count(&self) -> usize {
        self.column_count
    }

    /// Prepare [`crate:Document`]
    pub fn prepare_document(&self, document: &mut Document) {
        document.preamble.use_package("tabularx");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_table_with_five_columns() {
        let table = Table::new(
            TableKind::Tabularx,
            String::from("textwidth"),
            String::from("llXrr"),
        );

        assert_eq!(table.column_count(), 5 as usize);
    }

    #[test]
    fn push_row_to_table() {
        let mut table = Table::new(
            TableKind::Tabularx,
            String::from("textwidth"),
            String::from("X"),
        );
        let cell = Cell {
            value: String::from("para"),
        };

        let row = Row {
            cells: vec![cell],
            is_first_header: false,
            is_header: false,
        };
        assert_eq!(table.column_count(), 1 as usize);
        assert_eq!(table.rows.len(), 0);
        table.push_row(row).unwrap();
        assert_eq!(table.rows.len(), 1);
    }

    #[test]
    fn push_wrong_number_of_row_to_table() {
        let mut table = Table::new(
            TableKind::Tabularx,
            String::from("textwidth"),
            String::from("lX"),
        );

        let mut row = Row::new();
        row.push_cell("row 1".to_string());

        assert_eq!(
            table.push_row(row),
            Err(TableError::WrongNumberOfColumns(1, 2))
        );
    }
}
