use anyhow::Result;

use crate::models::{SharedState, Todo};

// TODO: Refactor out of this?
#[derive(Debug)]
pub enum TodoRepoError {
    NotFound,
}

// TODO: Refactor name
#[derive(Clone)]
pub struct TodoRepo {
    state: SharedState,
}

// TODO: Refactor name
pub trait Repo {
    fn new(shared_state: SharedState) -> Self; // TOOD: borrow
    fn all(&self) -> Result<Vec<Todo>, TodoRepoError>;
    fn get(&self, id: String) -> Result<Todo, TodoRepoError>;
    fn create(&mut self, todo: &Todo) -> Result<Todo, TodoRepoError>;
    fn update(&self, todo: &Todo) -> Result<bool, TodoRepoError>; // TODO: update return to return Todo
    fn delete(&self, id: String) -> Result<bool, TodoRepoError>;
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

    fn all(&self) -> Result<Vec<Todo>, TodoRepoError> {
        Ok(self.state.read().unwrap().todos.clone())
    }

    fn get(&self, id: String) -> Result<Todo, TodoRepoError> {
        let index = self.get_index(id);
        let state = self.state.read().unwrap();
        match index {
            Some(index) => Ok(state.todos.get(index).unwrap().clone()),
            None => Err(TodoRepoError::NotFound),
        }
    }

    fn create(&mut self, todo: &Todo) -> Result<Todo, TodoRepoError> {
        self.state.write().unwrap().todos.push(todo.clone());
        Ok(todo.clone())
    }

    fn update(&self, todo: &Todo) -> Result<bool, TodoRepoError> {
        let id = todo.to_owned().id.to_string();
        let index = self.get_index(id.to_owned());
        if let Some(index) = index {
            let mut state = self.state.write().unwrap();
            state.todos.splice(index..=index, Some(todo.to_owned())); // TODO: refactor to_owned?
            return Ok(true);
        }
        return Err(TodoRepoError::NotFound);
    }

    fn delete(&self, id: String) -> Result<bool, TodoRepoError> {
        let index = self.get_index(id);
        let mut state = self.state.write().unwrap();

        if let Some(index) = index {
            state.todos.remove(index);
            return Ok(true);
        }
        return Err(TodoRepoError::NotFound);
    }
}
