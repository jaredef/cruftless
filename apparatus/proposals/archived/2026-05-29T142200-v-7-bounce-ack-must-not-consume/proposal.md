---
helmsman_session: helmsman-2026-05-29-principal
proposed_commits:
  - f2349c1d8457dd87413f22b91207bb8450de3836
target_branch: main
summary: §V.7 bounce-ack must not consume directives addressed to other instances (interim CAACP discipline per keeper Telegram 10372)
risk_class: apparatus
---
Doc-only update codifying the interim discipline that prevents recurrence of the bounce-ack-consumes-target failure mode (PIND ac77efff delivery miss). Non-target instances send a separate response message instead of ack-RESOLVED on the original. Structural fix (target_instance_id schema + endpoint enforcement) tracked separately. No substrate impact.
