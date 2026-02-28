# Mini-PLM system

## Current Architecture:
This is currently a hobby project and since im not a big fan of databases, I decided to store the data in a json format.

The part or object currently has this initial structure.
```
{
  "parts": [
    {
      "number": "P-1000",
      "lifecycle": "Design"
    }
  ],
  "changes": []
}
```

The parts and changes elements are two different lists.



## How to use:
The current commands are:
- create
- promote
- show
- list
- history
- help

### Create an object:
This command will create an object with an initial lifecycle of "Design".

The path only follows this path (Design -> Prototype -> Production -> Obsolete) 
This is currently hard coded and will be changed once a better idea comes up.
```
cargo run create "P-1000"
```

### Promote object
design -> prototype
```
cargo run promote --eco "EN1234567" --reason "Finished testing" P-1000 prototype
```

prototype -> production
```
cargo run promote --eco "EN1234568" --reason "PM approved" P-1000 production
```

production -> obsolete
```
cargo run promote --eco "EN1234569" --reason "Reached EOL" P-1000 obsolete
```

### Create another object:
```
cargo run create "P-1001"
```

### List all created objects:
```
cargo run list
```

### Show an object:
```
cargo run show P-1000
```

### Show the history of an object
```
cargo run history P-1000
```
