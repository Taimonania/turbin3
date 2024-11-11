```mermaid
sequenceDiagram
    participant User
    participant TimeTrigger
    participant Worker

    User ->> TimeTrigger: 1. Creates Trigger & Deposits Funds
    Worker ->> TimeTrigger: 1. Reads
    Worker ->> TimeTrigger: 2. Executes when valid

    User ->> TimeTrigger: 2. Pauses Trigger
    User ->> TimeTrigger: 3. Stops Trigger

    TimeTrigger ->> User: 3. Retrieves funds
```