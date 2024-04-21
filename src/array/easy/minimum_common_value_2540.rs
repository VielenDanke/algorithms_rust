pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let (mut left, mut right) = (0usize, 0usize);
    let (n1_size, n2_size) = (nums1.len(), nums2.len()) ;

    while left < n1_size && right < n2_size {
        if nums2[right] < nums1[left] {
            right += 1;
        } else if nums1[left] < nums2[right] {
            left += 1;
        } else {
            return nums1[left];
        }
    }
    -1
}
