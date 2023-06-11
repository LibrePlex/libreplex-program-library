
use std::cell::Ref;

use anchor_lang::prelude::*;

use anchor_lang::{AnchorDeserialize, AnchorSerialize};


#[repr(C)]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct AttributeMapping {
    pub current: u32,
    // this is to ensure that each attributemapping has the same length
    // for accessing with AccountLoader / working with slices
    pub max_onchain_attribute_count: u32,
    // if we are using onchain description 
    pub attributes: Vec<u8>
}



impl AttributeMapping {
    /// The size of entrants excluding the entrants array
    pub const BASE_SIZE: usize = 4 + 4;

    pub fn get_element_size(&self) -> usize {
        4 + self.max_onchain_attribute_count as usize
    }

    pub fn get_attribute(&self, attribute_mappings: Ref<&mut [u8]>, index: usize) -> &[u8] {
        let element_size = self.get_element_size(); 
        let start_index = AttributeMapping::BASE_SIZE + element_size * index;
        &self.attributes[start_index..start_index + element_size]
    }

    pub fn append_attribute_mapping(
        &mut self,
        // mut attribute_mappings: RefMut<&mut [u8]>,
        attribute_mapping: &[u8],
    ) -> Result<()> {
        let element_size = self.get_element_size();
        let current_index = AttributeMapping::BASE_SIZE + element_size * self.current as usize;
        let target_attribute_mapping: &mut [u8] = &mut self.attributes[current_index..current_index + element_size];
        target_attribute_mapping.copy_from_slice(attribute_mapping);
        self.current += 1;

        Ok(())
    }
}


