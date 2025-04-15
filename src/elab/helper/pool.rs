
pub struct Pool<K> {
    items: Vec<K>,
}

impl<K> Pool<K> {
    pub fn new() -> Self {
        Self { items: vec![] }
    }
    pub fn insert(&mut self, item: K) {
        self.items.push(item);
    }

    pub fn get_vec(&self) -> &Vec<K> {
        &self.items
    }

    pub fn get_item(&self, index: usize) -> Option<&K> {
        self.items.get(index)
    }

    pub(crate) unsafe fn from_ptr<MutPtr, KPtr>(
        ptr: MutPtr,
        len_f: impl Fn(MutPtr) -> usize,
        get_k_f: impl Fn(MutPtr, usize) -> KPtr
    ) -> Self
    where
        MutPtr: Copy,
        K: From<KPtr>
    {
        let len: usize = len_f(ptr);

        let mut pool: Pool<K> = Self::new();
        for index in 0..len {
            let k_ptr = get_k_f(ptr, index);
            let k_real: K = K::from(k_ptr);
            pool.insert(k_real);
        }
        pool
    }
}