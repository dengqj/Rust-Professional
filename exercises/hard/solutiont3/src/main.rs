// I AM NOT DONE

mod district;

fn main() {
    let provinces = district::count_provinces();
    println!("provinces: {provinces}");
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_provinces() {
        // Assuming you have a known input file "district.json" with a specific structure
        // and you know the expected number of provinces for that input.
        let expected_provinces = "3,3,2,2,1"; // Replace with the actual expected number of provinces

        let provinces = district::count_provinces();
        assert_eq!(provinces, expected_provinces);
    }
}