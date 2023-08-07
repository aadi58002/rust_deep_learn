struct Flatten<I>
where
    I: Iterator,
    I::Item: IntoIterator,
{
    outer: I,
    inner: Option<<I::Item as IntoIterator>::IntoIter>,
}

impl<I> Iterator for Flatten<I>
where
    I: Iterator,
    I::Item: IntoIterator,
{
    type Item = <I::Item as IntoIterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut next_inner) = self.inner {
                if let Some(val) = next_inner.next() {
                    return Some(val);
                };
                self.inner = None;
            }
            self.inner = Some(self.outer.next()?.into_iter());
        }
    }
}

fn main() {
    println!("Hello, world!");
    let testing = vec![vec![1, 2, 3, 4], vec![2, 3, 5, 6]];
    let test = Flatten {
        outer: testing.into_iter(),
        inner: None,
    };
    for val in test {
        println!("{:?}", val);
    }
}
