### Brief

Pick the best hand(s) from a list of poker hands.

See [wikipedia](https://en.wikipedia.org/wiki/List_of_poker_hands) for an
overview of poker hands.

## Hints

- Ranking a list of poker hands can be considered a sorting problem.
- Rust provides the [sort](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort) method for `Vec<T> where T: Ord`.
- [`Ord` types](https://doc.rust-lang.org/std/cmp/trait.Ord.html) form a [total order](https://en.wikipedia.org/wiki/Total_order): exactly one of `a < b`, `a == b`, or `a > b` must be true.
- Poker hands do not conform to a total order: it is possible for two hands to be non-equal but have equal sort order. Example: `"3S 4S 5D 6H JH"`, `"3H 4H 5C 6C JD"`.
- Rust provides the [`PartialOrd` trait](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html) to handle the case of sortable things which do not have a total order. However, it doesn't provide a standard `sort` method for `Vec<T> where T: PartialOrd`. The standard idiom to sort a vector in this case is `your_vec.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::{Less|Equal|Greater}));`, depending on your needs.
- You might consider implementing a type representing a poker hand which implements `PartialOrd`.


### Writing the Code

Execute the tests with:

```bash
$ cargo test
```

All but the first test have been ignored. After you get the first test to
pass, open the tests source file which is located in the `tests` directory
and remove the `#[ignore]` flag from the next test and get the tests to pass
again. Each separate test is a function with `#[test]` flag above it.
Continue, until you pass every test.

If you wish to run all ignored tests without editing the tests source file, use:

```bash
$ cargo test -- --ignored
```

To run a specific test, for example `some_test`, you can use:

```bash
$ cargo test some_test
```

If the specific test is ignored use:

```bash
$ cargo test some_test -- --ignored
```

To learn more about Rust tests refer to the [online test documentation][rust-tests]

Make sure to read the [Modules][modules] chapter if you
haven't already, it will help you with organizing your files.

## Further improvements

To format your solution, inside the solution directory use

```bash
cargo fmt
```

To see, if your solution contains some common ineffective use cases, inside the solution directory use

```bash
cargo clippy --all-targets
```

### Evaluation Criteria

- Rust best practices
- Show us your work through your commit history
- Completeness: did you complete the features? Are all the tests running?
- Correctness: does the functionality act in sensible, thought-out ways?
- Maintainability: is it written in a clean, maintainable way?


### CodeSubmit

Please organize, design, and document your code as if it were going into production - then push your changes 
to the master branch. After you have pushed your code, you may submit the assignment on the assignment page.

All the best and happy coding,

The CipherStash Team