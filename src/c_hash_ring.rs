use std::slice::from_raw_parts;
use libc::*;

pub struct HashRing {
    ptr: *mut HashRingRawPtr,
}

enum HashRingRawPtr {}

enum HashRingNodePtr {}

pub enum HashFunction {
    SHA1 = 1,
    MD5 = 2,
}

impl HashRing {
    pub fn hash_ring_create(num_replicas: u32, hash_function: HashFunction) -> Option<Self> {
        let hash_function: uint8_t = match hash_function {
            HashFunction::SHA1 => 1,
            HashFunction::MD5 => 2,
        };
        let ring = unsafe { hash_ring_create(num_replicas, hash_function) };
        if ring.is_null() {
            None
        } else {
            Some(HashRing { ptr: ring })
        }
    }
    pub fn hash_ring_add_node(&mut self, name: &[u8]) -> Result<(), ()> {
        let code = unsafe {
            hash_ring_add_node(self.ptr, name.as_ptr(), name.len() as uint32_t)
        };
        if code == 0 {
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn hash_ring_find_node(&self, key: &[u8]) -> Option<Vec<u8>> {
        let node = unsafe {
            hash_ring_find_node(self.ptr, key.as_ptr(), key.len() as uint32_t)
        };
        if node.is_null() {
            None
        } else {
            let key = unsafe {
                let key_ptr: *const u8 = get_hash_ring_node_name(node);
                let key_len = get_hash_ring_node_name_len(node);
                from_raw_parts(key_ptr, key_len as usize)
            };
            Some(key.to_vec())
        }
    }

    pub fn hash_ring_print(&self) {
        unsafe { hash_ring_print(self.ptr) };
    }
}

impl Drop for HashRing {
    fn drop(&mut self) {
        unsafe {
            hash_ring_free(self.ptr);
        }
    }
}

#[allow(improper_ctypes)]
extern {
    fn hash_ring_create(num_replicas: uint32_t, hash_function: uint8_t) -> *mut HashRingRawPtr;
    fn hash_ring_free(ring: *mut HashRingRawPtr) -> c_void;
    fn hash_ring_add_node(ring: *mut HashRingRawPtr, name: *const uint8_t, name_len: uint32_t) -> c_int;
    fn hash_ring_find_node(ring: *const HashRingRawPtr, key: *const uint8_t, key_len: uint32_t) -> *const HashRingNodePtr;
    fn hash_ring_print(ring: *const HashRingRawPtr) -> c_void;

    fn get_hash_ring_node_name(node: *const HashRingNodePtr) -> *const uint8_t;
    fn get_hash_ring_node_name_len(node: *const HashRingNodePtr) -> uint32_t;
}