use anyhow::Result;

use crate::{
    error::TodoError,
    models::{SharedState, Todo},
};

// TODO: refactor to use an in-memory db?

// TODO: Refactor name
#[derive(Clone)]
pub struct TodoRepo {
    state: SharedState,
}

// TODO: Refactor name
pub trait Repo {
    fn new(shared_state: SharedState) -> Self; // TOOD: borrow
    fn all(&self) -> Result<Vec<Todo>, TodoError>;
    fn get(&self, id: String) -> Result<Todo, TodoError>;
    fn create(&mut self, todo: &Todo) -> Result<Todo, TodoError>;
    fn update(&self, todo: &Todo) -> Result<bool, TodoError>; // TODO: update return to return Todo
    fn delete(&self, id: String) -> Result<bool, TodoError>;
}

impl TodoRepo {
    fn get_index(&self, id: String) -> Option<usize> {
        let state = self.state.read().unwrap();
        return Some(
            state
                .todos
                .iter()
                .position(|todo| todo.id.to_string() == id)
                .unwrap(),
        );
    }
}

impl Repo for TodoRepo {
    fn new(shared_state: SharedState) -> Self {
        TodoRepo {
            state: shared_state,
        }
    }

    fn all(&self) -> Result<Vec<Todo>, TodoError> {
        let read_state = self.state.read();
        return match read_state {
            Ok(state) => Ok(state.todos.clone()),
            Err(_err) => Err(TodoError::FailedToGetLock),
        };
    }

    fn get(&self, id: String) -> Result<Todo, TodoError> {
        let index = self.get_index(id);
        match index {
            Some(index) => {
                let read_state = self.state.read();
                return match read_state {
                    Ok(state) => {
                        if let Some(todo) = state.todos.get(index) {
                            return Ok(todo.clone());
                        }
                        return Err(TodoError::NotFound);
                    }
                    Err(_err) => Err(TodoError::FailedToGetLock),
                };
            }
            None => Err(TodoError::NotFound),
        }
    }

    fn create(&mut self, todo: &Todo) -> Result<Todo, TodoError> {
        let write_state = self.state.write();
        return match write_state {
            Ok(mut state) => {
                state.todos.push(todo.clone());
                return Ok(todo.clone());
            }
            Err(_err) => Err(TodoError::FailedToGetLock),
        };
    }

    fn update(&self, todo: &Todo) -> Result<bool, TodoError> {
        let id = todo.to_owned().id.to_string();
        let index = self.get_index(id.to_owned());
        return match index {
            Some(index) => {
                let write_state = self.state.write();
                if let Ok(mut state) = write_state {
                    state.todos.splice(index..=index, Some(todo.to_owned())); // TODO: refactor to_owned?
                    return Ok(true);
                }
                return Err(TodoError::FailedToGetLock);
            }
            None => Err(TodoError::NotFound),
        };
    }

    fn delete(&self, id: String) -> Result<bool, TodoError> {
        let index = self.get_index(id);
        return match index {
            Some(index) => {
                let write_state = self.state.write();
                if let Ok(mut state) = write_state {
                    state.todos.remove(index);
                    return Ok(true);
                }
                return Err(TodoError::NotFound);
            }
            None => Err(TodoError::NotFound),
        };
    }
}
