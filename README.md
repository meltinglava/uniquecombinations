# Project Title

This crate give you all the combinations of values in a vec

## Example

```rs
let actual: Vec<_> = Combinations::new(vec![1, 2, 2, 3, 4], 3).collect();
let expected = vec![
    vec![1, 2, 2],
    vec![1, 2, 3],
    vec![1, 2, 4],
    vec![1, 3, 4],
    vec![2, 2, 3],
    vec![2, 2, 4],
    vec![2, 3, 4],
];
assert!(actual == expected);

```

## Contributing

Any contributions are welcomed.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.
