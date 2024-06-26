
# LOL Arena Stat Anvil Calculator

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Modules](#modules)
  - [main.rs](#mainrs)
  - [lib.rs](#librs)
  - [player_stats.rs](#player_statsrs)
  - [stat.rs](#statrs)
  - [stat_shard.rs](#stat_shardrs)
  - [anvil_stat_analyzer.rs](#anvil_stat_analyzers)
  - [anvil_stat_calculator.rs](#anvil_stat_calculators)

## Introduction

The LOL Arena Stat Anvil Calculator is a command-line application that simulates the process of rolling and selecting stat shards for a player in a game. This simulation helps to calculate the total gold value of the player's stats and allows for various interactions such as rolling new shards, resetting the stats, or stopping the simulation.

## Features

- Roll for new stat shards and choose from three options.
- Calculate the total gold value of the current stats.
- Display current stats and shard roll count.
- Reset the stats to start over.
- Interactive menu for user actions.
- Advanced analysis of rolled stats and their priority selection.

## Installation

1. Ensure you have Rust installed on your system. If not, install it from [rust-lang.org](https://www.rust-lang.org/).
2. Download the project files:
    - Click [here](https://github.com/gqsnt/lol_arena_stat_anvil_calculator/archive/refs/heads/master.zip) to download the ZIP file.
    - Or clone the repository using Git:
    ```sh
    git clone https://github.com/gqsnt/lol_arena_stat_anvil_calculator
    ```
3. Navigate to the project directory:
    ```sh
    cd lol_arena_stat_anvil_calculator
    ```
4. Build the project:
    ```sh
    cargo build --release
    ```

## Usage

The project includes two binaries: `anvil_stat_calculator` and `anvil_stat_analyzer`.

### anvil_stat_calculator

This interactive command-line tool allows users to roll for stat shards, reset stats, and stop the simulation through a menu-based interface.

1. Run the application:
    ```sh
    cargo run --bin anvil_stat_calculator --release
    ```
2. Follow the prompts to interact with the simulator:
    - Choose "Roll" to roll for new stat shards.
    - Choose "Reset" to reset the current stats.
    - Choose "Stop" to exit the application.

### anvil_stat_analyzer

This module provides an advanced analysis of rolled stats over multiple rounds, selecting high-priority shards and analyzing the results.

1. Run the application:
    ```sh
    cargo run --bin anvil_stat_analyzer --release
    ```
2. The application will automatically perform the analysis based on the predefined configuration and display the results.

## Modules

### main.rs

The main entry point of the application. It initializes the `PlayerStats` structure and provides an interactive menu for the user to roll stat shards, reset stats, or stop the application.

### lib.rs

This module serves as the library root and includes references to the `stat`, `stat_shard`, and `player_stats` modules.

### player_stats.rs

Defines the `PlayerStats` structure and its associated methods. This module handles the player's stats, including rolling for new stat shards, calculating total gold value, inserting stats and shards, and resetting stats.

### stat.rs

Defines the `Stat` enumeration, which represents various stats a player can have. This module includes methods for getting the gold value of a stat and the min-max range for stat values.

### stat_shard.rs

Defines the `StatShard` enumeration, which represents different types of stat shards. This module includes methods for generating all variants of stat shards, including the special "Pristine" shard.

### anvil_stat_analyzer.rs

This module provides an advanced analysis of rolled stats over multiple rounds, selecting high-priority shards and analyzing the results.

### anvil_stat_calculator.rs

An interactive command-line tool that allows users to roll for stat shards, reset stats, and stop the simulation through a menu-based interface.
