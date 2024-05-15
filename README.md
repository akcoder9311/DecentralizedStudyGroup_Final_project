# StudyGroupContract

A smart contract for managing study groups on the Soroban platform. This contract allows users to create, join, contribute to, and leave study groups. It also supports reward distribution based on contributions.

## Features

- **Create Group**: Create a new study group with a title, description, and initial contributors.
- **Join Group**: Join an existing study group.
- **Contribute**: Contribute to a study group.
- **View Group**: View details of a study group.
- **Distribute Rewards**: Distribute rewards to participants based on their contributions.

### Storage Keys

- `GroupID`: Used to store the unique identifier for groups.

### Data Structures

- `StudyGroup`: Represents a study group with fields for creator, title, description, participants, and contributions.
- `Contribution`: Represents a contribution made by a participant with fields for contributor and amount.


### Functions

- `create_group`: Creates a new study group.
- `join_group`: Allows a participant to join an existing study group.
- `contribute`: Allows a participant to contribute to a study group.
- `distribute_rewards`: Distributes rewards to participants based on their contributions.
- `view_group`: Retrieves the details of a study group.


## Getting Started

### Prerequisites

- Rust and Cargo installed
- Soroban SDK installed

### Installation

1. Clone the repository:
   ```sh
   git clone https://github.com/yourusername/StudyGroupContract.git
   cd StudyGroupContract
2.

## Project Structure

```text
.
├── contracts
│   └── ds_graoup
│       ├── src
│       │   ├── lib.rs
│       │   └── test.rs
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```

