//! Private terminal command editing and parsing.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum UiCommand {
    Exit,
    Pause,
    Resume,
    ForceExit,
    Interrupt,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// An edit to a terminal command line.
pub enum EditAction {
    /// Insert a Unicode character at the cursor.
    Insert(char),
    /// Remove the preceding character.
    Backspace,
    /// Remove the character at the cursor.
    Delete,
    /// Move the cursor one character left.
    Left,
    /// Move the cursor one character right.
    Right,
    /// Move to the start of the line.
    Home,
    /// Move to the end of the line.
    End,
    /// Empty the line and reset the cursor.
    Clear,
    /// Submit the current line and clear the editor.
    Submit,
}

#[derive(Debug, Eq, PartialEq)]
pub(super) enum CommandSubmission {
    Empty,
    Parsed(UiCommand),
    Unknown(String),
}

#[derive(Default)]
/// A Unicode-aware command-line editor with a character-position cursor.
pub struct CommandInput {
    characters: Vec<char>,
    cursor: usize,
}

impl CommandInput {
    /// Apply an edit without interpreting its contents as a Workflow command.
    /// Read [`Self::text`] before clearing a line to implement a custom parser.
    #[cfg(feature = "external-tools")]
    pub fn edit_text(&mut self, action: EditAction) {
        if action == EditAction::Submit {
            self.clear();
        } else {
            let _ = self.edit(action);
        }
    }

    pub(super) fn edit(&mut self, action: EditAction) -> Option<CommandSubmission> {
        match action {
            EditAction::Insert(character) => {
                self.characters.insert(self.cursor, character);
                self.cursor += 1;
            }
            EditAction::Backspace if self.cursor > 0 => {
                self.cursor -= 1;
                self.characters.remove(self.cursor);
            }
            EditAction::Delete if self.cursor < self.characters.len() => {
                self.characters.remove(self.cursor);
            }
            EditAction::Left => self.cursor = self.cursor.saturating_sub(1),
            EditAction::Right => self.cursor = (self.cursor + 1).min(self.characters.len()),
            EditAction::Home => self.cursor = 0,
            EditAction::End => self.cursor = self.characters.len(),
            EditAction::Clear => self.clear(),
            EditAction::Submit => {
                let command = self.text();
                self.clear();
                return Some(parse(&command));
            }
            _ => {}
        }
        None
    }

    /// Return the current contents.
    pub fn text(&self) -> String {
        self.characters.iter().collect()
    }

    /// Return the cursor's zero-based character position.
    pub const fn cursor(&self) -> usize {
        self.cursor
    }

    /// Empty the editor and reset its cursor.
    pub fn clear(&mut self) {
        self.characters.clear();
        self.cursor = 0;
    }
}

fn parse(input: &str) -> CommandSubmission {
    match input.trim() {
        "" => CommandSubmission::Empty,
        "pause" => CommandSubmission::Parsed(UiCommand::Pause),
        "resume" => CommandSubmission::Parsed(UiCommand::Resume),
        "exit" => CommandSubmission::Parsed(UiCommand::Exit),
        "exit --force" => CommandSubmission::Parsed(UiCommand::ForceExit),
        unknown => CommandSubmission::Unknown(unknown.to_owned()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn editor_supports_insertion_navigation_and_exit_submission() {
        let mut command = CommandInput::default();
        for character in ['e', 'x', 't'] {
            assert_eq!(command.edit(EditAction::Insert(character)), None);
        }
        command.edit(EditAction::Left);
        command.edit(EditAction::Insert('i'));
        assert_eq!(command.text(), "exit");
        assert_eq!(command.cursor(), 3);
        assert_eq!(
            command.edit(EditAction::Submit),
            Some(CommandSubmission::Parsed(UiCommand::Exit))
        );
        assert_eq!(command.text(), "");
    }

    #[test]
    fn parser_preserves_the_former_exact_command_contract() {
        assert_eq!(
            parse(" exit \t"),
            CommandSubmission::Parsed(UiCommand::Exit)
        );
        assert_eq!(
            parse(" exit --force \t"),
            CommandSubmission::Parsed(UiCommand::ForceExit)
        );
        assert_eq!(parse(""), CommandSubmission::Empty);
        assert_eq!(
            parse("status"),
            CommandSubmission::Unknown("status".to_owned())
        );
        assert_eq!(parse("EXIT"), CommandSubmission::Unknown("EXIT".to_owned()));
    }
}
