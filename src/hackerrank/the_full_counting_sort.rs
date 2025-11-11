/*
 * Complete the 'countSort' function below.
 *
 * The function accepts 2D_STRING_ARRAY arr as parameter.
 */

fn countSort(arr: &[Vec<String>]) {
    // 1. Get n (number of elements) from the input array
    let n = arr.len();

    // 2. Determine the midpoint
    // Integer division correctly handles both even and odd `n`.
    let mid = n / 2;

    // 3. Initialize the buckets for counting sort.
    // We assume the integer keys are in the range 0..=99, which is
    // standard for this problem.
    const MAX_KEY: usize = 100;
    // Create 100 empty "buckets", each being a Vec<String>
    let mut buckets: Vec<Vec<String>> = vec![Vec::new(); MAX_KEY];

    // 4. Iterate through the input array and place strings into buckets
    // We use .enumerate() to get the index `i` to check against `mid`.
    for (i, item) in arr.iter().enumerate() {

        // item[0] is the key (as a String), item[1] is the value (as a String)
        if item.len() < 2 {
            continue; // Skip malformed input lines
        }

        // Parse the key into a number
        let key: usize = item[0].parse().expect("Failed to parse key");
        let value = &item[1]; // Get a reference to the string value

        // 5. Apply the "twist":
        // If the item is in the first half (i < mid), use "-".
        // Otherwise, use the actual string.
        let processed_value = if i < mid {
            "-".to_string()
        } else {
            value.clone() // We must clone the string to own it in the bucket
        };

        // 6. Add the processed string to its corresponding bucket.
        // Because we `push` (append) in the original input order (0..n),
        // we guarantee stability for items with the same key.
        if key < MAX_KEY {
            buckets[key].push(processed_value);
        } else {
            // Handle cases where key might be out of assumed range
            eprintln!("Warning: key {} is out of expected range [0, {})", key, MAX_KEY);
        }
    }

    // 7. Build the final output
    // We can pre-allocate the result vector with size `n` for efficiency.
    let mut result: Vec<String> = Vec::with_capacity(n);

    // Iterate through the buckets in sorted order (from 0 to 99)
    for bucket in buckets {
        // `extend` moves all elements from `bucket` to the end of `result`.
        // This concatenates all buckets in sorted order.
        result.extend(bucket);
    }

    // 8. Print the final result as a single space-separated string
    println!("{}", result.join(" "));
}