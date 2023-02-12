use bevy::reflect::*;

use crate::prelude::*;

//#########################################################################
// Reflect
//#########################################################################
impl<T: FromReflect> GetTypeRegistration for Grid<T> {
    fn get_type_registration() -> TypeRegistration {
        let mut registration = TypeRegistration::of::<Grid<T>>();
        registration.insert::<ReflectFromPtr>(FromType::<Grid<T>>::from_type());
        let ignored_indices = [].into_iter();
        registration
            .insert::<serde::SerializationData>(serde::SerializationData::new(ignored_indices));
        registration
    }
}

impl<T: FromReflect> Typed for Grid<T> {
    fn type_info() -> &'static TypeInfo {
        static CELL: utility::GenericTypeInfoCell = utility::GenericTypeInfoCell::new();

        CELL.get_or_insert::<Self, _>(|| {
            let fields = [
                NamedField::new::<UVec2>("dimensions"),
                NamedField::new::<Vec<T>>("cells"),
            ];
            let info = StructInfo::new::<Self>("Grid", &fields);
            TypeInfo::Struct(info)
        })
    }
}

impl<T: FromReflect> Struct for Grid<T> {
    fn field(&self, name: &str) -> Option<&dyn Reflect> {
        match name {
            "dimensions" => Some(&self.dimensions),
            "cells" => Some(&self.cells),
            _ => None,
        }
    }

    fn field_mut(&mut self, name: &str) -> Option<&mut dyn Reflect> {
        match name {
            "dimensions" => Some(&mut self.dimensions),
            "cells" => Some(&mut self.cells),
            _ => None,
        }
    }

    fn field_at(&self, index: usize) -> Option<&dyn Reflect> {
        match index {
            0_usize => Some(&self.dimensions),
            1_usize => Some(&self.cells),
            _ => None,
        }
    }

    fn field_at_mut(&mut self, index: usize) -> Option<&mut dyn Reflect> {
        match index {
            0_usize => Some(&mut self.dimensions),
            1_usize => Some(&mut self.cells),
            _ => None,
        }
    }

    fn name_at(&self, index: usize) -> Option<&str> {
        match index {
            0_usize => Some("dimensions"),
            1_usize => Some("cells"),
            _ => None,
        }
    }

    fn field_len(&self) -> usize {
        2usize
    }

    fn iter_fields(&self) -> FieldIter {
        FieldIter::new(self)
    }

    fn clone_dynamic(&self) -> DynamicStruct {
        let mut dynamic = DynamicStruct::default();
        dynamic.set_name(self.type_name().to_string());
        dynamic.insert_boxed("dimensions", self.dimensions.clone_value());
        dynamic.insert_boxed("cells", self.cells.clone_value());
        dynamic
    }
}

impl<T: FromReflect> Reflect for Grid<T> {
    #[inline]
    fn type_name(&self) -> &str {
        std::any::type_name::<Self>()
    }

    #[inline]
    fn get_type_info(&self) -> &'static TypeInfo {
        <Self as Typed>::type_info()
    }

    #[inline]
    fn into_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }

    #[inline]
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    #[inline]
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    #[inline]
    fn into_reflect(self: Box<Self>) -> Box<dyn Reflect> {
        self
    }

    #[inline]
    fn as_reflect(&self) -> &dyn Reflect {
        self
    }

    #[inline]
    fn as_reflect_mut(&mut self) -> &mut dyn Reflect {
        self
    }

    #[inline]
    fn clone_value(&self) -> Box<dyn Reflect> {
        Box::new(Struct::clone_dynamic(self))
    }

    #[inline]
    fn set(&mut self, value: Box<dyn Reflect>) -> Result<(), Box<dyn Reflect>> {
        *self = value.take()?;
        Ok(())
    }

    #[inline]
    fn apply(&mut self, value: &dyn Reflect) {
        if let ReflectRef::Struct(struct_value) = value.reflect_ref() {
            for (i, value) in struct_value.iter_fields().enumerate() {
                let name = struct_value.name_at(i).unwrap();
                if let Some(v) = Struct::field_mut(self, name) {
                    v.apply(value)
                }
            }
        } else {
            panic!("Attempted to apply non-struct type to struct type: {value:?}");
        }
    }

    fn reflect_ref(&self) -> ReflectRef {
        ReflectRef::Struct(self)
    }

    fn reflect_mut(&mut self) -> ReflectMut {
        ReflectMut::Struct(self)
    }

    fn reflect_owned(self: Box<Self>) -> ReflectOwned {
        ReflectOwned::Struct(self)
    }

    fn reflect_partial_eq(&self, value: &dyn Reflect) -> Option<bool> {
        struct_partial_eq(self, value)
    }
}
