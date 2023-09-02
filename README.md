# timetable-solver

Generate the most optimal timetables for a given set of classes and their respective shifts.

The generated timetables contain a single shift for each class.

| Input                                                                                                            | Output                                                                                                           |
|------------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------|
| ![image](https://github.com/chicoferreira/timetable-solver/assets/36338391/d997916a-bd4a-48bb-b2c1-3d7358778213) | ![image](https://github.com/chicoferreira/timetable-solver/assets/36338391/502518b9-9ced-4576-aa70-40f8df0ea9f1) | 

## How it works

After reading and parsing
the [schedule input file](schedule.toml), the program will
generate every possible timetable for the given classes and their respective shifts.

It generates the timetables applying the [cartesian product](https://en.wikipedia.org/wiki/Cartesian_product) for the
shifts in each class.

The timetables are then separated by the number of week days that have classes.

From the timetables with one to five week days, the program selects the timetables with the shortest time elapsed
between the first and last class, filtering timetables with overlaps.

In the example above, you can choose the timetable with two days of classes, but you will have to wait 3 (total) hours
between classes.

You can also choose the timetable with three days of classes and don't have to wait between classes or more if you don't
want to have too many classes in a single day.

## Running

### Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [Git](https://git-scm.com/downloads)

### Steps

1. Clone the repository using `git clone https://github.com/chicoferreira/timetable-solver`.
2. Navigate to the project folder using `cd timetable-solver`.
3. Write your schedule in `schedule.toml` according to the format in the file.
4. Run the program using `cargo run` or compile it using `cargo build` and run the binary in `target/debug/timetable-solver`.
5. The generated timetables will be printed to standard output.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.