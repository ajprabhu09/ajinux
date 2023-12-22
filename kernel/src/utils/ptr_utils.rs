pub fn as_ref<'a, T>(ptr: *mut T) -> Option<&'a T> {
    return unsafe { ptr.as_ref() };
}

pub fn as_ref_mut<'a, T>(ptr: *mut T) -> Option<&'a mut T> {
    return unsafe { ptr.as_mut() };
}
