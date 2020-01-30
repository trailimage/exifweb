struct Post {
    title: str,
    photos: Vec<&Photo>,
    next: &Post,
    prev: &Post,
}
