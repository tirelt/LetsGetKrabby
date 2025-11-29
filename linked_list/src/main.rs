struct LinkedList<'a,T> {
    head: Option<&'a T>,
    tail: Option<Box<LinkedList<'a,T>>>
}
            
impl<'a,T> LinkedList<'a,T>{
    fn new()->LinkedList<'a,T>{
        LinkedList{
            head: None,
            tail: None,
        }
    }
    fn push(&mut self,el: &'a T){
        match self.head {
            None => self.head = Some(el),
            Some(_) => {
                if self.tail.is_none() {
                    self.tail = Some(Box::new(LinkedList::new())); 
                }
                self.tail.as_mut().unwrap().push(el);
            }
        }
    }
    fn len(&self)->u32{
        match self.head {
            None => 0,
            Some(_) => 1 + match &self.tail {
                None => 0,
                Some(t) => t.len(),
            }
        }
    }
    fn pop(&mut self){
        match self.head {
            None => (),
            Some(_) => {
                if self.tail.is_none() {
                    self.head = None;
                } else {
                    self.tail.as_mut().unwrap().pop();
                }
            }
        }
    }
}
fn main() {
    let mut list: LinkedList<'_, String> = LinkedList::new();
    let first = String::from("Theo");
    let second = String::from("Tirel");
    let third = String::from("le boss");
    list.push(&first);
    let mut size = list.len();
    println!("{size}");
    list.push(&second);
    list.push(&third);
    size = list.len();
    println!("{size}");
    list.pop();
    size = list.len();
    println!("{size}");
}
