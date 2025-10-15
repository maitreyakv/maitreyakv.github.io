#[derive(Debug, Clone)]
pub struct Typer {
    left: Vec<char>,
    right: Vec<char>,
    is_cursor_visible: bool,
}
impl Typer {
    pub fn empty() -> Self {
        Self {
            left: Vec::new(),
            right: Vec::new(),
            is_cursor_visible: true,
        }
    }

    pub fn step(self, step: TyperStep) -> Self {
        match step {
            TyperStep::NoOp => self,
            TyperStep::Forward => self.step_forward(),
            TyperStep::Backward => self.step_backward(),
            TyperStep::SetRight(text) => Self {
                right: text.chars().rev().collect(),
                ..self
            },
            TyperStep::FlipCursorVisibility => Self {
                is_cursor_visible: !self.is_cursor_visible,
                ..self
            },
        }
    }

    pub fn is_cursor_visible(&self) -> bool {
        self.is_cursor_visible
    }

    pub fn left_as_string_with_space_for_cursor(&self) -> String {
        let mut left_string = self.left.iter().collect::<String>();
        if !self.is_at_right_end() {
            left_string.pop();
        }
        left_string
    }

    pub fn left_as_string(&self) -> String {
        self.left.iter().collect::<String>()
    }

    pub fn right_as_string(&self) -> String {
        self.right.iter().rev().collect()
    }

    pub fn is_at_right_end(&self) -> bool {
        self.right.is_empty()
    }

    // TODO: Check for tail all optimization???
    fn step_forward(mut self) -> Self {
        match self.right.pop() {
            None => self,
            Some(c) if c == '\t' || c == '\n' => {
                self.left.push(c);
                self.step_forward()
            }
            Some(c) => {
                self.left.push(c);
                self
            }
        }
    }

    // TODO: Check for tail all optimization???
    fn step_backward(mut self) -> Self {
        match self.left.pop() {
            None => self,
            Some(c) if c == '\t' || c == '\n' => {
                self.right.push(c);
                self.step_backward()
            }
            Some(c) => {
                self.right.push(c);
                self
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum TyperStep {
    NoOp,
    Forward,
    Backward,
    SetRight(String),
    FlipCursorVisibility,
}
