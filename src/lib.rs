extern crate libc;

pub mod c_hash_ring;

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use ::c_hash_ring;
        let mut ring = c_hash_ring::HashRing::hash_ring_create(8, c_hash_ring::HashFunction::SHA1).unwrap();
        let slot_a = "slotA".as_bytes();
        let slot_b = "slotB".as_bytes();

        let key_a = "keyA".as_bytes();
        let key_b = "keyBBBB".as_bytes();
        let key_c = "keyB_".as_bytes();

        ring.hash_ring_add_node(slot_a).unwrap();
        ring.hash_ring_add_node(slot_b).unwrap();

        let node = ring.hash_ring_find_node(key_a);
        assert_eq!(node, Some(slot_a.to_vec()));

        let node = ring.hash_ring_find_node(key_b);
        assert_eq!(node, Some(slot_a.to_vec()));

        let node = ring.hash_ring_find_node(key_c);
        assert_eq!(node, Some(slot_b.to_vec()));
    }
}