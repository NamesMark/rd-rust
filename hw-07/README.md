# Homework for Lesson #7 - Multithreading + i/o

## Problem
- Application should run once if the arguments were provided, or ask for them interactively if not.
- Set up concurrency using `std::mpsc::channel` with two threads: producer (obtains user input) and consumer (processes input and returns output).
- Read csv either from a file or from the console.

## Approach
### Two execution modes
Depending on the number of arguments, I determine whether we are in the interactive or one-shot mode:
```
let execution_mode = match args.len() {
    1 => ExecutionMode::Interactive,
    _ => ExecutionMode::OneShot,
};
```

### Concurrency
1. I set up two channels: one for sending input data, and another for syncing when the processing has ended (to ensure proper order of input-output in the console)
```
let (tx, rx) = channel(); // data channel
let (done_tx, done_rx) = channel(); // sync channel
```
2. Two threads that receive input and process data separately
```
let producer = thread::spawn( {... });
let consumer = thread::spawn(move || {...});
```
3. The producer sends data to the consumer: `tx.send((command, subcommand, input))`
4. The consumer receives and executes the command on the data: `rx.recv() {execute_command(command, subcommand, input);`
5. Then the consumer sends the signal that the processing has ended to the producer, so that we can resume receiving input from the user: `done_tx.send(()).unwrap();`
6. The producer receives the signal: `if done_rx.recv()`
7. In the end we join the threads to ensure that they fully execute before the main thread exits:
```
producer.join().unwrap();
consumer.join().unwrap();
```
### Reading from file:
```
let mut file = File::open(path).expect("Unable to open the file");
let mut content = String::new();
file.read_to_string(&mut content).expect("Unable to read the file");
Ok(content)
```

## Usage
There are two modes of operation: one-time that uses the provided arguments, and interactive, that relies on user input through the console.

### One-shot mode
#### Text commands
To run the program in this mode, the user has to provide a valid command, e.g.: `cargo run leetify`. All the text commands work the same as in previous versions (double new line is needed to execute):
```
> cargo run uppercase
Please enter your input to uppercase:
> Hello
>
HELLO
```
#### Csv processing
If only "csv" was provided, the program will attempt to read the file from the default test file `test/username.csv`.
```
PS C:\Prog\Courses\r_d\rd-rust\hw-07> cargo run csv
╭────────────────┬────────────────┬────────────────┬────────────────╮
│    Username    │   Identifier   │   First name   │   Last name    │
╞════════════════╪════════════════╪════════════════╪════════════════╡
│    booker12    ┆      9012      ┆     Rachel     ┆     Booker     │
...
```

### Interactive mode

To enter the interactive mode, the user has to run `cargo run` without additional arguments.

#### Text commands
Supports the whole list of commands.
```
Please enter a command:
> leetify
Please enter your input to leetify:
> A Hermit Crab
> Finds a slab
> For tourists wish to grab
>
4 H3RM17 CR48
F1ND5 4 5L48
F0R 70UR1575 W15H 70 9R48
```

#### Csv processing

Accessed using `csv` command when prompted.

- If no additional settings were provided, the user is prompted to enter their csv data into the console
```
❯ cargo run
Please enter a command:
> csv
Please enter your CSV settings: p:<path> d:<delimiter> w:<max_column_width> 
(leave empty to enter in console):
>   
Please enter your input to csv:
> Username; Identifier;First name;Last name
> booker12;9012;Rachel;Booker
> grey07;2070;Laura;Grey
> johnson81;4081;Craig;Johnson
> jenkins46;9346;Mary;Jenkins
> smith79;5079;Jamie;Smith
>
╭────────────────┬────────────────┬────────────────┬────────────────╮
│    Username    │   Identifier   │   First name   │   Last name    │
╞════════════════╪════════════════╪════════════════╪════════════════╡
│    booker12    ┆      9012      ┆     Rachel     ┆     Booker     │
```
- Alternatively, the user can specify their settings as follows:

```
Please enter a command:
> csv
Please enter your CSV settings: p:<path> d:<delimiter> w:<max_column_width> 
(leave empty to enter in console):
> p:test/username.csv d:semicolon
╭────────────────┬────────────────┬────────────────┬────────────────╮
│    Username    │   Identifier   │   First name   │   Last name    │
╞════════════════╪════════════════╪════════════════╪════════════════╡
│    booker12    ┆      9012      ┆     Rachel     ┆     Booker     │
```

- The user can choose between `d:colon` and `d:semicolon` delimiters:

```
Please enter a command:
> csv
Please enter your CSV settings: p:<path> d:<delimiter> w:<max_column_width> 
(leave empty to enter in console):
> p:test/username.csv d:comma
╭────────────────╮
│Username; Identi│
╞════════════════╡
│booker12;9012;Ra│
│────────────────│
│grey07;2070;Laur│
```

- The user can choose the maximum allowed length for header name and data:
```
Please enter your CSV settings: p:<path> d:<delimiter> w:<max_column_width> 
(leave empty to enter in console):
> p:test/username.csv w:4
╭────────────────┬────────────────┬────────────────┬────────────────╮
│      User      │       Ide      │      Firs      │      Last      │
╞════════════════╪════════════════╪════════════════╪════════════════╡
│      book      ┆      9012      ┆      Rach      ┆      Book      │
│────────────────┼────────────────┼────────────────┼────────────────│
│      grey      ┆      2070      ┆      Laur      ┆      Grey      │
```

* Note: currently it doesn't support word-wrapping, so unexpectedly lengthy inputs will break the output. 

## List of commands
Text processing:
- lowercase: convert the entire text to lowercase.
- uppercase: convert the entire text to uppercase.
- no-spaces: remove all spaces from the text.
- slugify: convert the text into a slug (a version of the text suitable for URLs) using the slug crate.
- short-slugify: convert the text into a short slug (similar to slugify but with a max length, cropped to the last dash before the length threshold).
- alternating: convert the text to an alternating between uppercase and lowercase pattern using the convert_case crate.
- leetify: Convert the text to leet speak using a .map() and a match block over specific letters.

Csv:
- csv: parse the test as a CSV and print the data as a table. Usage: p:<path> d:<delimiter> w:<max_column_width>.

Defaults: 
- path: test/username.csv
- delimiter: semicolon
- width: 16

