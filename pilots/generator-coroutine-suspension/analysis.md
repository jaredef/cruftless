# generator-coroutine-suspension — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| generator-suspension | FAIL | Mechanism gap #3: eager-collect prevents lazy yield/resume |
| generators | PASS | Basic generator iteration works via eager-collect; lazy semantics do not |
| async-iteration | PASS | Async iteration protocol works; sync generator suspension is the gap |
| iteration-protocol | PASS | Iterator protocol shape correct; generator-specific suspend/resume missing |

This locale targets mechanism gap #3 (lazy generator suspension), the highest-leverage single compiler candidate at 1,492 test262 rows. The generator-suspension FAIL fixture is the direct anchor. The generators PASS fixture confirms the eager-collect path works for finite generators; the gap is next(val)/throw(err) delivery and infinite generators.
