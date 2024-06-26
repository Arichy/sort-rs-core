use napi::bindgen_prelude::{Float64Array, Uint32Array};
use napi_derive::napi;
use rayon::join;

use super::partition::partition_common;
use super::traits::{Sizable, Splitable};
// use super::traits::{IndexComparable, Sizable, Splitable, Swappable};
use super::universal_struct::UniversalStruct;

#[napi]
fn quick_sort_objects_common(
  mut priority_list: Vec<Float64Array>,
  mut index_list: Uint32Array,
  order_list: Vec<bool>,
  rayon: bool,
) {
  let mut result: Vec<&mut [f64]> = Vec::with_capacity(priority_list.len());
  for priority in priority_list.iter_mut() {
    result.push(priority);
  }

  let structure = UniversalStruct {
    priority_list: &mut result,
    index_list: &mut index_list,
    order_list: &order_list,
  };

  if rayon {
    _rayon_quick_sort_objects_common(structure);
  } else {
    _normal_quick_sort_objects_common(structure);
  }
}

fn _normal_quick_sort_objects_common<'origin: 'priority_list_ref, 'priority_list_ref>(
  mut universal_struct: UniversalStruct<'origin, 'priority_list_ref>,
) {
  if universal_struct.len() <= 1 {
    return;
  }

  let order_list = universal_struct.order_list;

  let mid = partition_common(&mut universal_struct);

  let ((mut left_priority_list, left_index_list), (mut right_priority_list, right_index_list)) =
    universal_struct.split_at_mut(mid);

  let left_struct = UniversalStruct {
    priority_list: &mut left_priority_list,
    index_list: left_index_list,
    order_list: order_list,
  };

  let right_struct = UniversalStruct {
    priority_list: &mut right_priority_list,
    index_list: right_index_list,
    order_list: order_list,
  };
  _normal_quick_sort_objects_common(left_struct);
  _normal_quick_sort_objects_common(right_struct);
}

fn _rayon_quick_sort_objects_common<'origin: 'priority_list_ref, 'priority_list_ref>(
  mut universal_struct: UniversalStruct<'origin, 'priority_list_ref>,
) {
  if universal_struct.len() <= 1 {
    return;
  }

  let order_list = universal_struct.order_list;

  let mid = partition_common(&mut universal_struct);

  let ((mut left_priority_list, left_index_list), (mut right_priority_list, right_index_list)) =
    universal_struct.split_at_mut(mid);

  let left_struct = UniversalStruct {
    priority_list: &mut left_priority_list,
    index_list: left_index_list,
    order_list: order_list,
  };

  let right_struct = UniversalStruct {
    priority_list: &mut right_priority_list,
    index_list: right_index_list,
    order_list: order_list,
  };

  join(
    || _normal_quick_sort_objects_common(left_struct),
    || _normal_quick_sort_objects_common(right_struct),
  );
}
