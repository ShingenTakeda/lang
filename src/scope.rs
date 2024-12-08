use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::instruction::{Instruction, StackValue};

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<String>,
    pub block: Vec<Instruction>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Function { scope: Box<Scope>, func: Function },
    Value { value: StackValue },
}

#[derive(Debug, PartialEq, Clone)]
pub struct Scope {
    store: Rc<RefCell<HashMap<String, Object>>>,
    outter_scope: Option<Box<Scope>>,
    func_id: String,
}

impl Scope {
    pub fn new() -> Self {
        Scope {
            store: Rc::new(RefCell::new(HashMap::new())),
            outter_scope: None,
            func_id: "root".to_string(),
        }
    }

    pub fn get_store_len(&self) -> usize {
        self.store.borrow().len()
    }

    pub fn from_scope(func_id: String, outer_scope: Scope) -> Self {
        let scope = Scope {
            store: Rc::new(RefCell::new(HashMap::new())),
            outter_scope: Some(Box::new(outer_scope)),
            func_id,
        };
        return scope;
    }

    pub fn dec_var(&self, id: String, val: StackValue) -> Option<Object> {
        self.store
            .borrow_mut()
            .insert(id, Object::Value { value: val })
    }

    pub fn dec_func(
        &self,
        name: String,
        params: Vec<String>,
        block: Vec<Instruction>,
    ) -> Option<Object> {
        self.store.borrow_mut().insert(
            name.clone(),
            Object::Function {
                scope: Box::new(self.clone()),
                func: Function {
                    name: name.clone(),
                    params: params,
                    block: block,
                },
            },
        )
    }

    pub fn set_var(&self, id: String, val: StackValue) -> Option<StackValue> {
        let scope = self.store.borrow().clone();
        match scope.get(&id.clone()) {
            Some(..) => {
                self.store
                    .borrow_mut()
                    .insert(id, Object::Value { value: val.clone() });
                Some(val)
            }
            None => match self.outter_scope.clone() {
                Some(out) => out.set_var(id, val),
                None => None,
            },
        }
    }

    pub fn get_var(&self, id: String) -> Option<Object> {
        let scope = self.store.borrow();
        let var = scope.get(&id.clone());
        match var {
            Some(x) => Some(x.clone()),
            None => match self.outter_scope.clone() {
                Some(out) => out.get_var(id),
                None => None,
            },
        }
    }
}
