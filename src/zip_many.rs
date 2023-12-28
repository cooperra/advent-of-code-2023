pub fn zip_many<I, O, T>(iters: impl IntoIterator<Item = I>) -> ZipMany<O, T>
where
    I: IntoIterator<Item = T, IntoIter = O>,
    O: Iterator<Item = T>,
{
    ZipMany {
        iters: iters.into_iter().map(|item| item.into_iter()).collect(),
    }
}

pub struct ZipMany<I, T>
where
    I: Iterator<Item = T>,
{
    iters: Vec<I>,
}

impl<I, T> Iterator for ZipMany<I, T>
where
    I: Iterator<Item = T>,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut nexts = Vec::new();
        for iter in self.iters.iter_mut() {
            if let Some(next) = iter.next() {
                nexts.push(next);
            } else {
                return None;
            }
        }
        return Some(nexts);
    }
}
