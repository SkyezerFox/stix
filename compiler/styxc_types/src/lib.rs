use std::{error::Error, fmt::Debug, str::FromStr};

#[derive(Debug, Clone)]
pub enum Type {
    /// Represents a 64-bit integer type.
    Int,
    /// Represents a 64-bit floating point type.
    Float,
    /// Represents a boolean type.
    Bool,
    /// Represents a character type.
    Char,
    /// Represents a string type.
    String,
    /// Represents a tuple type.
    Tuple(Vec<Type>),
    /// Represents an array type.
    Array(Box<Type>),
    /// Represents a map type.
    Map(Box<Type>, Box<Type>),
    /// Represents a set type.
    Set(Box<Type>),
    /// Represents an optional type.
    Optional(Box<Type>),
    /// Represents a union of types.
    Union(Vec<Type>),
    /// Represents an intersection type.
    Intersection(Vec<Type>),
    /// Represents a type already referred to.
    Circular(Box<Type>),
    /// Represents a function type.
    Func(Vec<Type>, Box<Type>),
    /// Represents a reference to a named type.
    Reference(String),
    /// Represents a unit type.
    Unit,
    /// Represents a type that has yet to be inferred.
    Infer,
    /// Represents a type that can never occur.
    Never,
}

impl From<String> for Type {
    fn from(s: String) -> Self {
        use Type::*;
        match s.as_str() {
            "()" => Unit,
            "bool" => Bool,
            "char" => Char,
            "float" => Float,
            "int" => Int,
            "str" => String,
            _ => panic!("cannot convert non-primitive string representation of a type to a type"),
        }
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Type) -> bool {
        equate_types(self, other)
    }
}

impl FromStr for Type {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "()" => Type::Unit,
            "bool" => Type::Bool,
            "char" => Type::Char,
            "float" => Type::Float,
            "int" => Type::Int,
            "str" => Type::String,
            _ => Type::Reference(s.to_string()),
        })
    }
}

impl Type {
    /// Compute the intersection of this type with another.
    pub fn intersect(self, other: Type) -> Type {
        if self == other {
            return self;
        }

        Type::Never
    }
}

/// A trait implementable by function objects.
pub trait FuncType: Debug {
    /// Fetch the type of this function.
    fn as_ty(&self) -> Type;
    /// Fetch the argument types of this function.
    fn argument_types(&self) -> Vec<Type>;
    // /Fetch the return type of this function.
    fn ret_ty(&self) -> Type;
}

/// Test if an intersection type is valid.
pub fn validate_intersection(t: &Type) -> Result<(), Box<dyn Error>> {
    match t {
        Type::Intersection(types) => {
            // iterate over types
            for t in types {
                // test if type is a primitive
                if !is_primitive(t) {
                    continue;
                }
                return Err("Cannot compute intesection type of primitives".into());
            }
            return Ok(());
        }
        _ => Err("Cannot validate intersection type if it isn't an intersection type!".into()),
    }
}

/// Test if one type is included within another. Can be used to test for extension.
pub fn is_subtype(a: &Type, b: &Type) -> bool {
    // set a can never be a member of set a
    if equate_types(a, b) {
        return false;
    };

    match (a, b) {
        (a, Type::Union(types)) => return types.contains(&a),
        _ => false,
    }
}

/// Test if type `a` is equal to type `b`.
pub fn equate_types(a: &Type, b: &Type) -> bool {
    // test if can use primitive equality
    if is_primitive(a) && is_primitive(b) {
        return equate_primitives(a, b);
    };
    match (a, b) {
        _ => false,
    }
}

/// Test if type `t` is a primitive.
pub fn is_primitive(t: &Type) -> bool {
    use Type::*;
    match t {
        Int | Float | Bool | String => true,
        _ => false,
    }
}

/// Test if two primitive types are equal.
pub fn equate_primitives(a: &Type, b: &Type) -> bool {
    use Type::*;
    match (a, b) {
        (Int, Int) => true,
        (Float, Float) => true,
        (Bool, Bool) => true,
		(String, String) => true,
        _ => false,
    }
}

/// Validate if a type is valid for insertion into a map.
pub fn validate_map_insertion(k: &Type, v: &Type, map: &Type) -> bool {
    use Type::*;
    match map {
        Map(key, value) => is_subtype(key, k) && is_subtype(value, v),
        _ => false,
    }
}

/// Validate if a type is valid for insertion into a map.
pub fn validate_set_insertion(v: &Type, set: &Type) -> bool {
    use Type::*;
    match set {
        Set(value) => is_subtype(value, v),
        _ => false,
    }
}

/// Validate if a type is valid for insertion into a map.
pub fn validate_array_insertion(v: &Type, array: &Type) -> bool {
    use Type::*;
    match array {
        Array(value) => is_subtype(value, v),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_equality() {
        assert!(equate_types(&Type::Int, &Type::Int));
    }

    #[test]
    fn union_inclusion() {
        assert!(is_subtype(
            &Type::Int,
            &Type::Union(vec![Type::Int, Type::Float])
        ));
    }
}
