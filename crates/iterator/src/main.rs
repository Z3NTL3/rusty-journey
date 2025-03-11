#![allow(unused)]
use std::sync::atomic::AtomicU64;

#[derive(Debug)]
struct User<'a> {
    email: &'a str,
    age: u8
}

#[derive(Debug)]
struct Users<'a> {
    counter: AtomicU64, // todo
    items: Vec<User<'a>>
}

type BorrowIter<'a, 'b> = std::slice::Iter<'b, User<'a>>;
impl<'a,'b> Users<'a> {
    fn iter(&'b self) -> BorrowIter<'a, 'b> {
        self.items.iter()
    }

    fn iter_mut(&'b mut self) -> IterMut<'a, 'b> {
        IterMut { items: self.items.iter_mut() }
    }

    fn owned_iter(self) -> OwnedIter<'a> {
        OwnedIter { items: self.items.into_iter() }
    }
}

impl<'a, 'b> IntoIterator for &'b mut Users<'a> {
    type Item = &'b mut User<'a>;

    type IntoIter = IterMut<'a, 'b>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut{items: self.items.iter_mut()}
    }
}

impl<'a, 'b> IntoIterator for &'b Users<'a> {
    type Item = &'b User<'a>;

    type IntoIter = BorrowIter<'a, 'b>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}

struct OwnedIter<'a> {
    items: std::vec::IntoIter<User<'a>>
}

impl<'a> Iterator for OwnedIter<'a> {
    type Item = User<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.items.next()
    }
}
struct IterMut<'a, 'b> {
    items: std::slice::IterMut<'b, User<'a>>
}

impl<'a, 'b> Iterator for IterMut<'a, 'b> {
    type Item = &'b mut User<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.items.next()
    }
}

fn main() {
    let mut users = Users{
        counter:AtomicU64::new(0),
        items: vec![
            User{
                email: "yo@gmail.com",
                age: 1
            },
            User{
                email: "yo2@gmail.com",
                age: 2
            }
        ]
    };

    let b = (&mut users).into_iter();
    for user in  users.iter_mut() {
        user.age+=1;
    }

    for user in users.iter() {
        println!("user: {:?}", user)
    }
}