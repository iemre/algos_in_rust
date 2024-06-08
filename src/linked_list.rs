struct Node<'a, T> {
    val: &'a T,
    next: Option<&'a Node<'a, T>>
}

impl<'a, T> Node<'a, T> {
    fn iterate_and_apply(&self, f: fn(val: &T)) {
        let iter: &Node<'a, T> = self;
        f(self.val);
        match iter.next {
            Some(n) => {
                n.iterate_and_apply(f);
            },
            None => {}
        }
    }
}

#[test]
fn test_iter_apply<'a>() {
    let n = Node::<'a, i32>{ val: &55, 
        next: Some(
            &Node::<'a, i32>{
                val: &66, 
                next: None
            }
        )
    };
    n.iterate_and_apply(|v: &i32| {
        print!("{}\n", (*v)*2);
    });
}