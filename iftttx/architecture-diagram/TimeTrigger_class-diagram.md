::: mermaid
classDiagram
    class TimeTrigger {
        +owner: Pubkey
        +bump: u8
        +fee: u64
        +action
        +action_calldata
        +start_at
        +interval
        +stop_after
    }

:::