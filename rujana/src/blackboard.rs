use super::types::Timestamp;


pub trait RetrievableBlackboard {
    fn get_single<T>(tag : String) -> &T;
    fn get<T>(tag : String) -> Vec<&T>;
    fn get_metadata<T>(tag : String) -> T::Metadata; // TODO: Will this work?
}

pub trait InsertableBlackboard {

    fn insert_single<T>(t : T) -> ();
    fn insert<T>(ts : Vec<T>) -> ();
}

struct Blackboard {

}

impl Blackboard {
    fn register(f : Factory, tag : String) -> () {}
}

impl InsertableBlackboard for Blackboard {
    fn insert_single<T>(t : T) -> () {}
    fn insert<T>(ts : Vec<T>) -> () {}
}

//impl RetrievableBlackboard for Blackboard {

//}


    // TODO: Can we use a drain iterator instead?
