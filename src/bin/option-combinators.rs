fn main() {
    let a_vec: Vec<Option<i32>> = vec![Some(1), None];

    for a in a_vec {
        dbg!(a);

        let a_is_some = a.is_some();
        dbg!(a_is_some);

        let a_is_none = a.is_none();
        dbg!(a_is_none);

        let a_mapped = a.map(|num| num + 1);
        dbg!(a_mapped);

        let a_filtered = a.filter(|predicate| *predicate == 1);
        dbg!(a_filtered);

        let a_or_else = a.or_else(|| Some(5));
        dbg!(a_or_else);

        let unwraped = a.unwrap_or_else(|| 6);
        dbg!(unwraped);
    }
    /*
    [src/bin/option-combinators.rs:4] a = Some(
        1,
    )
    [src/bin/option-combinators.rs:7] a_is_some = true
    [src/bin/option-combinators.rs:10] a_is_none = false
    [src/bin/option-combinators.rs:13] a_mapped = Some(
        2,
    )
    [src/bin/option-combinators.rs:16] a_filtered = Some(
        1,
    )
    [src/bin/option-combinators.rs:19] a_or_else = Some(
        1,
    )
    [src/bin/option-combinators.rs:22] unwraped = 1
     */
}

#[cfg(test)]
mod test {

    #[test]
    fn option_conbinators() {
        let a: Option<i32> = Some(1);

        assert_eq!(a.is_some(), true);

        assert_eq!(a.is_none(), false);

        assert_eq!(a.map(|num| num + 1), Some(2));

        assert_eq!(a.filter(|num| *num == 1), Some(1));

        assert_eq!(a.or_else(|| Some(5)), Some(1));

        assert_eq!(a.unwrap_or_else(|| 6), 1);
    }
}
