type N = u64;
type VecN = Vec<N>;

fn main() {
    let numbers: VecN = vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24];

    // Exercise using clone and returning tuple
    let sum_of_nums: N = sum(numbers.clone());
    let (product_of_nums, nums): (N, VecN) = product(numbers);
    let Ret {
        average_of_nums,
        nums: nums2,
    } = average(nums);
    // Doesn't actually calc average of products as nums is just passthrough
    let average_of_products: N = average(product(nums2).1).average_of_nums;

    println!("Sum of these numbers: {sum_of_nums}");
    println!("Product of these numbers: {product_of_nums}");
    println!("Average of these numbers: {average_of_nums}");
    println!("Average of nums post product: {average_of_products}");
}

fn sum(numbers: VecN) -> N {
    let mut total: N = 0;

    for num in numbers.iter() {
        total += num;
    }

    total
}

fn product(numbers: VecN) -> (N, VecN) {
    let mut total = 1;

    for num in numbers.iter() {
        total *= num
    }

    (total, numbers)
}

struct Ret {
    average_of_nums: N,
    nums: VecN,
}

fn average(numbers: VecN) -> Ret {
    let length = numbers.len() as N;

    Ret {
        average_of_nums: sum(numbers.clone()) / length,
        nums: numbers,
    }
}
