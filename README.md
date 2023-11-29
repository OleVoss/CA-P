# CA-P

CellularAutomata - Playground (CA-P) is a simulation for exploring and experimenting different CA patterns in different
dimensions. The applicaion is written in **rust** using the **bevy gameengine**.

Since the project is used to practice and explore the language and framework, development is dependent on time and
interest in the topic. Therefore, the state of the simulation is very fundamental and not really useful.

### Usage

To run the application just clone the repo and start it with `cargo run`.

![config_screenshot.png](doc%2Fconfig_screenshot.png)

| Config    | Usage                                                     |
|-----------|-----------------------------------------------------------|
| Dimension | `not implemented`                                         |
| Size      | Used to set the size of the universe                      |
| Noise     | Ratio of living cells, when population the universe       |
| Rule      | Selection which rule should be used within the simulation |

To populate the universe, press `n`.

### Rules

Rules are stored as assets in `CA-P/assets/rules.ron`. \
To add a custom rule, just edit the file and restart the application.
```
Rule (
    name: "New Rule",
    dimension: D2, // Dimension (only D2 is implemented at the moment)
    birth: [2, 5], // Number of neighbours required to come to live
    survival: [4], // Number of neighbours required to survive
)
```
