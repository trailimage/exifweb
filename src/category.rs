mod Post;

/// Post category.
struct Category {
   key: str,
   posts: Vec<&Post>,
}

impl Category {
   fn add(c: &Category) {}

   fn remove_post(p: &Post) {}

   fn is_parent() -> bool {
      true
   }

   fn is_child() -> bool {
      false
   }
}
