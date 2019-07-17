pub mod zero_init {
    pub fn initialize_to_zero<F: num_traits::Zero>(buffers: &mut [&mut [F]]) {
        for buffer in buffers.iter_mut() {
            for sample in buffer.iter_mut() {
                *sample = F::zero();
            }
        }
    }
}
