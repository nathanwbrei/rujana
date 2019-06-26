


pub trait Expert<T> {
    fn process(&self, blackboard : &mut Blackboard, output : &mut Vec<T>) -> ();
    fn update(&self, blackboard: &mut Blackboard) -> ();
    // TODO: Figure out Metadata
    // TODO: Figure out how to declare inputs
}

pub trait RetrievableBlackboard {
    fn get_single<T>(tag : String) -> & 'static T;
    fn get<'a,T>(tag : String) -> Vec<&'a T>;
}

pub trait InsertableBlackboard {

    fn insert_single<T>(t : T) -> ();
    fn insert<T>(ts : Vec<T>) -> ();
}

pub struct Blackboard {
    // Probably gonna be a map<typeid<T>, Box<T>
}

impl Blackboard {
    fn register<T>(expert : &mut Expert<T>, tag : String) -> () {}
}

impl InsertableBlackboard for Blackboard {
    fn insert_single<T>(t : T) -> () {}
    fn insert<T>(ts : Vec<T>) -> () {}
}

//impl RetrievableBlackboard for Blackboard {

//}

// TODO: Can we use a drain iterator instead?
