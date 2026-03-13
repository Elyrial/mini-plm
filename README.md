# Mini-PLM

A hobby project for managing parts through a lifecycle. Data is stored as JSON.

## Data Structure

Parts and changes are stored as two separate lists:

```json
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

Lifecycle stages follow a fixed path: **Design → Prototype → Production → Obsolete**

---

## CLI Commands

| Command          | Description                                          |
|------------------|------------------------------------------------------|
| `create`         | Create a new part                                    |
| `promote`        | Advance a part to the next lifecycle stage           |
| `change-order`   | Record an ECO against a part without stage promotion |
| `show`           | Show a part's current state                          |
| `list`           | List all parts                                       |
| `history`        | Show a part's change history                         |
| `serve`          | Start the web UI server                              |
| `help`           | Show help                                            |

### Create a part

New parts start in the `Design` stage.

```
cargo run create "P-1000"
```

### Promote a part

```
cargo run promote --eco "EN1234567" --reason "Finished testing" P-1000 prototype
cargo run promote --eco "EN1234568" --reason "PM approved"      P-1000 production
cargo run promote --eco "EN1234569" --reason "Reached EOL"      P-1000 obsolete
```

### Record a change order

Records an ECO audit entry against a part without changing its lifecycle stage. Use this when something about the part changes (BOM items, tolerances, notes) but no promotion is needed.

```
cargo run change-order P-1000 --eco "EN1234570" --reason "Updated BOM revision"
```

The history entry will show `from` and `to` as the same stage, distinguishing it from a promotion.

### List all parts

```
cargo run list
```

### Show a part

```
cargo run show P-1000
```

### Show change history

```
cargo run history P-1000
```

---

## Web UI

Start the server on the default port (3000):

```
cargo run serve
```

Or specify a custom port:

```
cargo run serve --port 8080
```

Then open `http://localhost:3000` in your browser. The UI shows all parts in a table, click a row to expand its change history.
