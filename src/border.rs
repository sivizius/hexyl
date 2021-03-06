#[derive(Clone, Copy)]
pub(crate) struct BorderElements {
    pub(crate) left_corner:      char,
    pub(crate) horizontal_line:  char,
    pub(crate) column_separator: char,
    pub(crate) right_corner:     char,
}

/// Style of the Border arround bytes and characters.
#[derive(Clone, Copy)]
pub enum BorderStyle {
    /// Use special unicode characters for border.
    /// This is the nicest one.
    Unicode,
    /// Use simple ascii characters for border.
    /// Looks okish.
    Ascii,
    /// No nation, no border.
    None,
}

const LOOKUP_HEADER: [Option<BorderElements>; 3] = [
    Some(BorderElements {
        left_corner:      '┌',
        horizontal_line:  '─',
        column_separator: '┬',
        right_corner:     '┐',
    }),
    Some(BorderElements {
        left_corner:      '+',
        horizontal_line:  '-',
        column_separator: '+',
        right_corner:     '+',
    }),
    None,
];
const LOOKUP_FOOTER: [Option<BorderElements>; 3] = [
    Some(BorderElements {
        left_corner:      '└',
        horizontal_line:  '─',
        column_separator: '┴',
        right_corner:     '┘',
    }),
    Some(BorderElements {
        left_corner:      '+',
        horizontal_line:  '-',
        column_separator: '+',
        right_corner:     '+',
    }),
    None,
];
const LOOKUP_OUTER_SEP: [char; 3] = ['│', '|', ' '];
const LOOKUP_INNER_SEP: [char; 3] = ['┊', '|', ' '];

impl BorderStyle {
    pub(crate) fn header_elems(&self) -> Option<BorderElements> {
        LOOKUP_HEADER[*self as usize]
    }

    pub(crate) fn footer_elems(&self) -> Option<BorderElements> {
        LOOKUP_FOOTER[*self as usize]
    }

    pub(crate) fn outer_sep(&self) -> char {
        LOOKUP_OUTER_SEP[*self as usize]
    }

    pub(crate) fn inner_sep(&self) -> char {
        LOOKUP_INNER_SEP[*self as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::{BorderElements, BorderStyle};

    fn helper(style: BorderStyle, expection: String) {
        let header = if let Some(header) = style.header_elems() {
            header
        } else {
            BorderElements {
                left_corner:      '1',
                horizontal_line:  '2',
                column_separator: '3',
                right_corner:     '4',
            }
        };
        let footer = if let Some(footer) = style.footer_elems() {
            footer
        } else {
            BorderElements {
                left_corner:      '5',
                horizontal_line:  '6',
                column_separator: '7',
                right_corner:     '8',
            }
        };
        let outer  = style.outer_sep();
        let inner  = style.inner_sep();
        assert_eq! (
            format! (
                "{hlc}{hhl}{hcs}{hhl}{hrc}\n{os} {is} {os}\n{flc}{fhl}{fcs}{fhl}{frc}",
                hlc = header.left_corner,
                hhl = header.horizontal_line,
                hcs = header.column_separator,
                hrc = header.right_corner,
                os  = outer,
                is  = inner,
                flc = footer.left_corner,
                fhl = footer.horizontal_line,
                fcs = footer.column_separator,
                frc = footer.right_corner,
            ),
            expection,
        )
    }

    #[test]
    fn unicode() {
        helper (
            BorderStyle::Unicode,
            format! (
                "{}\n{}\n{}",
                "┌─┬─┐",
                "│ ┊ │",
                "└─┴─┘",
            )
        );
    }

    #[test]
    fn ascii() {
        helper (
            BorderStyle::Ascii,
            format! (
                "{}\n{}\n{}",
                "+-+-+",
                "| | |",
                "+-+-+",
            )
        );
    }

    #[test]
    fn none() {
        helper (
            BorderStyle::None,
            format! (
                "{}\n{}\n{}",
                "12324",
                "     ",
                "56768",
            )
        );
    }
}
