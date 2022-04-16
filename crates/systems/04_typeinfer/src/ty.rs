use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Handler {
    pub input: Type,
    pub output: Type,
}

pub type Id = usize;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Effect {
    pub input: Type,
    pub output: Type,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Type {
    Number,
    String,
    Product(Vec<Self>),
    Sum(Vec<Self>),
    Function {
        parameter: Box<Self>,
        body: Box<Self>,
    },
    Array(Box<Self>),
    Set(Box<Self>),
    Variable(Id),
    ForAll {
        variable: Id,
        body: Box<Self>,
    },
    Existential(Id),
    Infer(Id),
    Effectful {
        ty: Box<Self>,
        effects: Vec<Effect>,
    },
    Brand {
        brand: String,
        item: Box<Self>,
    },
    Label {
        label: String,
        item: Box<Self>,
    },
}

pub(crate) trait TypeVisitorMut {
    fn visit_number(&mut self) {}
    fn visit_string(&mut self) {}
    fn visit_product(&mut self, types: &mut Vec<Type>) {
        types.iter_mut().for_each(|ty| self.visit(ty))
    }
    fn visit_sum(&mut self, types: &mut Vec<Type>) {
        types.iter_mut().for_each(|ty| self.visit(ty))
    }
    fn visit_function(&mut self, parameter: &mut Type, body: &mut Type) {
        self.visit(parameter);
        self.visit(body);
    }
    fn visit_array(&mut self, ty: &mut Type) {
        self.visit(ty);
    }
    fn visit_set(&mut self, ty: &mut Type) {
        self.visit(ty);
    }
    fn visit_variable(&mut self, _id: &mut Id) {}
    fn visit_forall(&mut self, _variable: &mut Id, body: &mut Type) {
        self.visit(body);
    }
    fn visit_existential(&mut self, _id: &mut Id) {}
    fn visit_infer(&mut self, _id: &mut Id) {}
    fn visit_effectful(&mut self, ty: &mut Type, effects: &mut Vec<Effect>) {
        self.visit(ty);
        effects
            .iter_mut()
            .for_each(|effect| self.visit_effect(effect))
    }
    fn visit_effect(&mut self, effect: &mut Effect) {
        self.visit(&mut effect.input);
        self.visit(&mut effect.output);
    }
    fn visit_brand(&mut self, _brand: &mut String, item: &mut Type) {
        self.visit(item);
    }
    fn visit_label(&mut self, _label: &mut String, item: &mut Type) {
        self.visit(item);
    }
    fn visit(&mut self, ty: &mut Type) {
        self.visit_inner(ty)
    }
    fn visit_inner(&mut self, ty: &mut Type) {
        match ty {
            Type::Number => self.visit_number(),
            Type::String => self.visit_string(),
            Type::Product(types) => self.visit_product(types),
            Type::Sum(types) => self.visit_sum(types),
            Type::Function { parameter, body } => self.visit_function(parameter, body),
            Type::Array(ty) => self.visit_array(ty),
            Type::Set(ty) => self.visit_set(ty),
            Type::Variable(id) => self.visit_variable(id),
            Type::ForAll { variable, body } => self.visit_forall(variable, body),
            Type::Existential(id) => self.visit_existential(id),
            Type::Infer(id) => self.visit_infer(id),
            Type::Effectful { ty, effects } => self.visit_effectful(ty, effects),
            Type::Brand { brand, item } => self.visit_brand(brand, item),
            Type::Label { label, item } => self.visit_label(label, item),
        }
    }
}

pub(crate) trait TypeVisitor {
    fn visit_number(&mut self) {}
    fn visit_string(&mut self) {}
    fn visit_product(&mut self, types: &Vec<Type>) {
        types.iter().for_each(|ty| self.visit(ty))
    }
    fn visit_sum(&mut self, types: &Vec<Type>) {
        types.iter().for_each(|ty| self.visit(ty))
    }
    fn visit_function(&mut self, parameter: &Type, body: &Type) {
        self.visit(parameter);
        self.visit(body);
    }
    fn visit_array(&mut self, ty: &Type) {
        self.visit(ty);
    }
    fn visit_set(&mut self, ty: &Type) {
        self.visit(ty);
    }
    fn visit_variable(&mut self, _id: &Id) {}
    fn visit_forall(&mut self, _variable: &Id, body: &Type) {
        self.visit(body);
    }
    fn visit_existential(&mut self, _id: &Id) {}
    fn visit_infer(&mut self, _id: &Id) {}
    fn visit_effectful(&mut self, ty: &Type, effects: &Vec<Effect>) {
        self.visit(ty);
        effects.iter().for_each(|effect| self.visit_effect(effect))
    }
    fn visit_effect(&mut self, effect: &Effect) {
        self.visit(&effect.input);
        self.visit(&effect.output);
    }
    fn visit_brand(&mut self, _brand: &String, item: &Type) {
        self.visit(item);
    }
    fn visit_label(&mut self, _label: &String, item: &Type) {
        self.visit(item);
    }
    fn visit(&mut self, ty: &Type) {
        self.visit_inner(ty)
    }
    fn visit_inner(&mut self, ty: &Type) {
        match ty {
            Type::Number => self.visit_number(),
            Type::String => self.visit_string(),
            Type::Product(types) => self.visit_product(types),
            Type::Sum(types) => self.visit_sum(types),
            Type::Function { parameter, body } => self.visit_function(parameter, body),
            Type::Array(ty) => self.visit_array(ty),
            Type::Set(ty) => self.visit_set(ty),
            Type::Variable(id) => self.visit_variable(id),
            Type::ForAll { variable, body } => self.visit_forall(variable, body),
            Type::Existential(id) => self.visit_existential(id),
            Type::Infer(id) => self.visit_infer(id),
            Type::Effectful { ty, effects } => self.visit_effectful(ty, effects),
            Type::Brand { brand, item } => self.visit_brand(brand, item),
            Type::Label { label, item } => self.visit_label(label, item),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct ExprTypes {
    pub types: HashMap<Id, Type>,
}

mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
