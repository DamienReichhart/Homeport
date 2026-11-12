//! The order of the rows: the cycle of a header, the stable arrangement,
//! and text compared as TanStack compares it.

use std::cmp::Ordering;

use crate::ui::widgets::table::sort::{arrange, natural, natural_chunks, next, Order, Sort};

#[test]
fn a_header_cycles_through_both_ways_and_back_to_none() {
    assert_eq!(next(None, false), Some(Order::Ascending));
    assert_eq!(next(Some(Order::Ascending), false), Some(Order::Descending));
    assert_eq!(next(Some(Order::Descending), false), None);
    // A column of numbers starts with the largest.
    assert_eq!(next(None, true), Some(Order::Descending));
    assert_eq!(next(Some(Order::Descending), true), Some(Order::Ascending));
    assert_eq!(next(Some(Order::Ascending), true), None);
}

#[test]
fn a_press_alone_sorts_by_that_column_only() {
    let mut sort = Sort::by("name", Order::Ascending);
    sort.toggle("size", true, false, false);
    assert_eq!(sort.keys(), [("size", Order::Descending)]);
    sort.toggle("size", true, false, false);
    assert_eq!(sort.keys(), [("size", Order::Ascending)]);
    sort.toggle("size", true, false, false);
    assert!(sort.is_empty());
}

#[test]
fn a_press_with_shift_adds_the_column_after_the_others() {
    let mut sort = Sort::by("status", Order::Ascending);
    sort.toggle("name", false, true, false);
    assert_eq!(
        sort.keys(),
        [("status", Order::Ascending), ("name", Order::Ascending)]
    );
    sort.toggle("status", false, true, false);
    assert_eq!(sort.of("status"), Some(Order::Descending));
    assert_eq!(sort.keys()[0].0, "status", "it keeps its place");
    sort.toggle("status", false, true, false);
    assert_eq!(sort.keys(), [("name", Order::Ascending)]);
}

#[test]
fn what_was_kept_names_a_column_once() {
    let sort = Sort::from_keys([
        ("name", Order::Ascending),
        ("name", Order::Descending),
        ("size", Order::Descending),
    ]);
    assert_eq!(
        sort.keys(),
        [("name", Order::Ascending), ("size", Order::Descending)]
    );
}

#[test]
fn text_sorts_whatever_its_case_and_digits_by_their_value() {
    let mut names = vec![
        "host-10", "Host-2", "alpha", "host-1", "Beta", "host", "7", "host-02a",
    ];
    names.sort_by(|a, b| natural(a, b));
    assert_eq!(
        names,
        ["alpha", "Beta", "host", "host-1", "Host-2", "host-02a", "host-10", "7"]
    );
    assert_eq!(natural("Prod", "prod"), Ordering::Equal);
    assert_eq!(natural("a", "B"), Ordering::Less, "not by code point");
    assert_eq!(natural("v9", "v10"), Ordering::Less);
    // Longer than any integer type, and still by value.
    assert_eq!(
        natural("id-99999999999999999999999", "id-100000000000000000000000"),
        Ordering::Less
    );
    assert_eq!(natural("10.0.0.9:2376", "10.0.0.10:2376"), Ordering::Less);
}

#[test]
fn ascii_and_other_text_sort_the_same_way() {
    // The same pairs through both paths.
    let names = [
        "", "a", "ab", "a1", "a01", "a1b", "a10", "A2", "1", "01", "10", "1a", "host", "host-",
        "host-2", "Host-10", "host-02a", "b",
    ];
    for one in names {
        for other in names {
            let plain = natural(one, other);
            let long = natural_chunks(one, other);
            assert_eq!(plain, long, "{one:?} against {other:?}");
            assert_eq!(plain, natural(other, one).reverse(), "{one:?} {other:?}");
        }
    }
}

#[test]
fn rows_that_compare_equal_keep_their_order() {
    let rows = [("b", 1), ("a", 2), ("b", 3), ("a", 4), ("c", 5)];
    let mut order: Vec<u32> = (0..5).collect();
    arrange(&mut order, &rows, |a, b| a.0.cmp(b.0));
    let sorted: Vec<i32> = order.iter().map(|&i| rows[i as usize].1).collect();
    assert_eq!(sorted, [2, 4, 1, 3, 5]);
    // Descending reverses the keys, not the rows that share one.
    arrange(&mut order, &rows, |a, b| {
        Order::Descending.apply(a.0.cmp(b.0))
    });
    let sorted: Vec<i32> = order.iter().map(|&i| rows[i as usize].1).collect();
    assert_eq!(sorted, [5, 1, 3, 2, 4]);
}

#[test]
fn the_arrangement_does_not_depend_on_where_it_started() {
    let rows = [3, 1, 2, 1, 3, 2];
    let mut from_start: Vec<u32> = (0..6).collect();
    let mut from_end: Vec<u32> = (0..6).rev().collect();
    arrange(&mut from_start, &rows, Ord::cmp);
    arrange(&mut from_end, &rows, Ord::cmp);
    assert_eq!(from_start, from_end);
    assert_eq!(from_start, [1, 3, 2, 5, 0, 4]);
}

#[test]
fn a_table_that_is_always_sorted_goes_from_one_way_to_the_other() {
    let mut sort = Sort::by("host", Order::Ascending);
    sort.toggle("host", false, false, true);
    assert_eq!(sort.keys(), [("host", Order::Descending)]);
    // Where the three states would unsort, it starts again.
    sort.toggle("host", false, false, true);
    assert_eq!(sort.keys(), [("host", Order::Ascending)]);
    // Another column: its first way, as `desc: false` in React.
    sort.toggle("name", false, false, true);
    assert_eq!(sort.keys(), [("name", Order::Ascending)]);
}
