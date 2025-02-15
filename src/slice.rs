pub trait StaticChunkable {
    type Value;

    fn static_chunks<'a, const N: usize>(
        &'a self,
    ) -> Option<impl Iterator<Item = &'a [Self::Value; N]>>
    where
        Self::Value: 'a;
}

impl<T> StaticChunkable for [T] {
    type Value = T;

    fn static_chunks<'a, const N: usize>(
        &'a self,
    ) -> Option<impl Iterator<Item = &'a [Self::Value; N]>>
    where
        Self::Value: 'a,
    {
        if self.len() % N == 0 {
            Some(
                self.chunks(N)
                    .map(|chunk| chunk.try_into().expect("unreachable")),
            )
        } else {
            None
        }
    }
}
