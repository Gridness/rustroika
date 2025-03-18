# Rustroika

A command-line tool to help Moscow residents decide whether to buy a monthly transport pass or pay per trip with their Troika card.

## Features

- Calculates the total cost of individual trips vs. monthly pass
- Accounts for the 50% discount on subsequent trips within 90 minutes
- Provides a clear recommendation on which payment method is more economical
- Beautiful colored output with clear visual presentation
- Handles rounding of discounted prices correctly

## Installation

### Prerequisites

- Rust toolchain (1.70.0 or later)
- Cargo (comes with Rust)

### Building from Source

1. Clone the repository:
```bash
git clone https://github.com/Gridness/rustroika.git
cd rustroika
```

2. Build the project:
```bash
cargo build --release
```

3. The executable will be available in `target/release/rustroika`

## Usage

```bash
rustroika --trips-week|-t <trips/week> --monthly-cost|-m <monthly_cost> --ticket-price|-p <ticket_price>
```

### Parameters

- `--trips-week` or `-t`: Number of trips you make per week
- `--monthly-cost` or `-m`: Cost of the monthly pass in RUB
- `--ticket-price` or `-p`: Cost of a single ticket in RUB

### Example

```bash
rustroika --trips-week 10 --monthly-cost 2900 --ticket-price 63
```

Or using short flags:
```bash
rustroika -t 10 -m 2900 -p 63
```

This example calculates costs for:
- 10 trips per week
- Monthly pass costing 2900 RUB
- Single ticket costing 63 RUB

### Output

The program will display:
- Total number of trips per month
- Monthly pass cost
- Individual trip costs (including discounts)
- Ticket price
- Clear recommendation on which option is cheaper
- Explanation of the discount logic

## How It Works

1. Calculates total monthly trips (weekly trips × 4)
2. Applies the 50% discount to subsequent trips within 90 minutes
3. Rounds down discounted prices (e.g., 63 → 31)
4. Compares total individual costs with monthly pass cost
5. Provides a recommendation with the amount saved

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the Apache 2.0 License - see the LICENSE file for details.
