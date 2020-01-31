use crate::post;

enum CategoryKind {
   Who(Vec<&Category>),
   What(Vec<&Category>),
   When(Vec<&Category>),
   Where(Vec<&Category>),
}

impl CategoryKind {
   fn add(&self) {
      //self.
   }

   pub fn get(&self, key_or_title: str) -> Option<&Category> {
      for i in &self.into_iter() {
         if i.title == key_or_title || i.key == key_or_title {
            return Some(i);
         }
      }
      return None;
   }

   //pub fn has()
}

// https://users.rust-lang.org/t/solved-unified-iteration-over-enum-of-vectors/11830/3
impl<'a> IntoIterator for &'a CategoryKind {
   type Item = &'a Category;
   type IntoIter = Box<Iterator<Item = &'a Category> + 'a>;

   fn into_iter(self) -> Self::IntoIter {
      match *self {
         CategoryKind::Who(ref k) => Box::new(k.into_iter().map(|x| x as &Category)),
         CategoryKind::What(ref k) => Box::new(k.into_iter().map(|x| x as &Category)),
         CategoryKind::When(ref k) => Box::new(k.into_iter().map(|x| x as &Category)),
         CategoryKind::Where(ref k) => Box::new(k.into_iter().map(|x| x as &Category)),
      }
   }
}

/// Post category.
#[derive(Clone, Debug)]
struct Category {
   key: str,
   title: str,
   posts: Vec<&Post>,
   kind: CategoryKind,
}

impl Category {
   fn add(&self, c: &Category) {}

   // fn remove_post(&self, p: &Post) {
   //    &self.posts.swap_remove(index: usize);
   // }
}
