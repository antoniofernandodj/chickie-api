// Define the ToProto trait

pub trait ToProto<T> {
    fn to_proto(&self) -> T;
}

// Em um módulo comum (como src/utils/proto_converter.rs)
pub fn to_proto_vec<T, P>(items: Vec<T>) -> Vec<P>
where
    T: ToProto<P>,
{
    items.into_iter().map(|item| item.to_proto()).collect()
}


