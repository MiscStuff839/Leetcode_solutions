use leetcode_derive::generate_list;

macro_rules! create_sol {
    ($name:ident) => {
        pub mod $name;
    }
}

generate_list!();

create_sol!(two_sum);
create_sol!(running_sum);
create_sol!(reverse_integer);
create_sol!(palindrome);
create_sol!(valid_palindrome);
create_sol!(roman_numeral);