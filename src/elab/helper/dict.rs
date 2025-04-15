use crate::bindings::{Wrapper_get_num_wires, Wrapper_get_wire_by_index, Yosys_RTLIL_Module};
use crate::elab::helper::id_string::IdString;
use crate::elab::wire::Wire;

pub struct Dict<T, K> {
    items: Vec<(T, K)>,
}

impl<T, K> Dict<T, K> {
    pub fn new() -> Self {
        Self { items: vec![] }
    }
    pub fn insert(&mut self, item: (T, K)) {
        self.items.push(item);
    }

    pub fn get_vec(&self) -> &Vec<(T, K)> {
        &self.items
    }

    pub fn get_item(&self, index: usize) -> Option<&(T, K)> {
        self.items.get(index)
    }

    pub(crate) unsafe fn from_ptr<MutPtr, TPtr, KPtr>(
        ptr: MutPtr,
        len_f: impl Fn(MutPtr) -> usize,
        get_t_f: impl Fn(MutPtr, usize) -> TPtr,
        get_k_f: impl Fn(MutPtr, usize) -> KPtr
    ) -> Self
    where
        MutPtr: Copy,
        T: From<TPtr>,
        K: From<KPtr>,
    {
        let len: usize = len_f(ptr);

        let mut dict: Dict<T, K> = Self::new();
        for index in 0..len {
            let t_ptr = get_t_f(ptr, index);
            let k_ptr = get_k_f(ptr, index);

            let t_real: T = T::from(t_ptr);
            let k_real: K = K::from(k_ptr);


            dict.insert((t_real, k_real));
        }
        dict
    }
}