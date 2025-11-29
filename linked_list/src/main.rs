struct LinkedList<'a,T> {
    head: Option<T>,
    tail: Option<&'a mut LinkedList<'a,T>>
}
impl<'a,T> LinkedList<'a,T>{
    fn new()->LinkedList<'a,T>{
        LinkedList{
            head: None,
            tail: None,
        }
    }
}
fn main() {
    let list: LinkedList<'_, String> = LinkedList::new();
}
