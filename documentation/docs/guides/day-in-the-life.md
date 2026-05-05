# A Day in the Life

Follow along with Alice, a backend engineer at a mid-sized startup, as she uses Stack throughout her workday. This narrative guide shows how the pieces fit together in a realistic workflow.

---

## Morning: Getting Started

### 8:55 AM — Arrive and Sync

Alice opens her terminal and checks the current state before starting work.

```bash
$ cd ~/work/api-service

$ gt sync
Fetching origin...
Restacking feature/payment-models onto main...
Restacking feature/payment-api onto feature/payment-models...
Restacking feature/payment-tests onto feature/payment-api...

Deleted 2 merged branches: feature/auth-models, feature/auth-api
Updated 3 PRs
```

Two PRs she submitted yesterday were merged overnight. Stack cleaned them up and rebased her remaining work automatically.

### 9:00 AM — Review Status

Alice checks what she's working on.

```bash
$ gt log
main
 └── feature/payment-models      #45 [open]
      └── feature/payment-api    #46 [open] [active]
           └── feature/payment-tests #47 [draft]
```

She's currently on `feature/payment-api`. The models PR (#45) is ready for review, the API PR (#46) is what she was working on yesterday, and the tests branch is still a draft.

She opens the TUI for a quick visual check:

```bash
$ gt tui
```

The TUI shows the same stack, plus MR status badges. She notices PR #45 has two approvals. She presses `q` to exit and lands it.

### 9:05 AM — Land an Approved PR

```bash
$ gt land feature/payment-models
Merging PR #45...
Branch feature/payment-models deleted.

$ gt sync
Restacking feature/payment-api onto main...
Restacking feature/payment-tests onto feature/payment-api...
Updated PR #46 base branch to main
```

PR #45 is merged. Stack updated #46 to target `main` instead of the deleted `feature/payment-models` branch.

---

## Mid-Morning: Starting New Work

### 9:30 AM — Create a New Stack

Alice's PM asks her to add webhook support. She creates a new stack for this work.

```bash
$ gt checkout main
$ gt create feature/webhook-models
Created and checked out feature/webhook-models
```

She writes the webhook data models and commits:

```bash
$ git add src/models/webhook.rs src/models/mod.rs
$ gt modify
Modified feature/webhook-models
```

She immediately stacks the next branch:

```bash
$ gt create feature/webhook-api
Created and checked out feature/webhook-api
```

She implements the API endpoints. Around 10:30 AM, she has a working endpoint and commits:

```bash
$ git add src/routes/webhook.rs src/routes/mod.rs
$ gt modify
Modified feature/webhook-api
```

### 10:35 AM — Submit Early

Even though the feature isn't complete, Alice submits what she has to get early feedback.

```bash
$ gt submit --from feature/webhook-models
Submitting feature/webhook-models...
  Created PR #48
Submitting feature/webhook-api...
  Created PR #49
```

PR #48 targets `main` and contains just the models. PR #49 targets `feature/webhook-models` and contains just the API endpoints. Each is ~150 lines — easy to review.

---

## Late Morning: Addressing Review Feedback

### 11:00 AM — Review Comments Arrive

Bob reviewed PR #48 and suggested adding a `created_at` field to the webhook model. Alice switches to that branch to make the change.

```bash
$ gt checkout feature/webhook-models
$ vim src/models/webhook.rs
# Add created_at field

$ git add src/models/webhook.rs
$ gt modify
Modified feature/webhook-models
Restacking feature/webhook-api onto feature/webhook-models...
```

`gt modify` automatically amended the branch and restacked `feature/webhook-api` on top of the updated model.

### 11:10 AM — Resubmit

```bash
$ gt submit
Updated PR #48
Updated PR #49
```

Both PRs are updated. Since PR #49 was rebased onto the modified PR #48, its diff is preserved and review comments are still anchored correctly.

---

## Lunch Break

### 12:30 PM — Quick Sync Before Lunch

Alice runs a quick sync to catch any midday merges:

```bash
$ gt sync
Already up to date.
```

Nothing new. She steps away.

---

## Afternoon: Deep Work

### 1:30 PM — Continue the Stack

After lunch, Alice adds the webhook delivery mechanism on top of the API branch:

```bash
$ gt checkout feature/webhook-api
$ gt create feature/webhook-delivery
Created and checked out feature/webhook-delivery
```

She implements the background job that actually sends webhooks. This is the most complex part — about 300 lines. She commits in two logical chunks:

```bash
$ git add src/jobs/webhook_delivery.rs
$ git commit -m "Add webhook delivery job"

$ git add src/jobs/webhook_retry.rs
$ git commit -m "Add webhook retry logic"
```

Stack tracks branches, not individual commits, so both commits are on `feature/webhook-delivery`.

### 2:45 PM — Submit the New Branch

```bash
$ gt submit --only feature/webhook-delivery
Submitting feature/webhook-delivery...
  Created PR #50
```

PR #50 targets `feature/webhook-api` and contains only the delivery logic. It's a 300-line PR, which is larger than ideal, but it's a cohesive unit of work.

---

## Late Afternoon: Landing and Cleanup

### 3:30 PM — PR Approved

While Alice was working on delivery, PR #48 (models) and PR #49 (API) were approved. She lands them in order.

```bash
$ gt land feature/webhook-models
Merging PR #48...
Branch feature/webhook-models deleted.

$ gt sync
Restacking feature/webhook-api onto main...
Restacking feature/webhook-delivery onto feature/webhook-api...
Updated PR #49 base branch to main
Updated PR #50 base branch to feature/webhook-api
```

### 3:35 PM — Land the Next One

```bash
$ gt land feature/webhook-api
Merging PR #49...
Branch feature/webhook-api deleted.

$ gt sync
Restacking feature/webhook-delivery onto main...
Updated PR #50 base branch to main
```

Now PR #50 (delivery) targets `main` directly. It's a single, reviewable PR that builds on work already merged.

### 3:40 PM — Check What's Left

```bash
$ gt log
main
 └── feature/webhook-delivery #50 [open] [active]
```

Alice's stack is down to one branch. She creates the final piece — tests:

```bash
$ gt create feature/webhook-tests
Created and checked out feature/webhook-tests
```

She writes integration tests for the full webhook flow. By 4:30 PM, she's ready to submit.

```bash
$ gt submit
Submitting feature/webhook-delivery...
  Updated PR #50
Submitting feature/webhook-tests...
  Created PR #51
```

---

## End of Day

### 5:00 PM — Final Sync and Review

Alice does a final sync and checks her open PRs.

```bash
$ gt sync
Already up to date.

$ gt log
main
 └── feature/webhook-delivery #50 [open]
      └── feature/webhook-tests #51 [draft] [active]
```

She opens the TUI to review everything at a glance:

```bash
$ gt tui
```

She navigates to the Stacks tab, presses `g` to refresh MR status from GitHub. PR #50 shows two approvals. PR #51 is still in draft.

She exits the TUI and decides to leave PR #51 as draft — she'll finish the tests tomorrow morning.

### 5:05 PM — Summary

Alice's day in numbers:

- **Syncs**: 3
- **Branches created**: 4
- **PRs submitted**: 4
- **PRs landed**: 3
- **Restacks**: 5 (all automatic)
- **Time spent on Git mechanics**: ~10 minutes
- **Time spent actually coding**: ~6 hours

---

## Key Patterns from Alice's Day

### 1. Sync First, Work Second

Alice always runs `gt sync` before starting work. This ensures she's building on the latest `main` and catches overnight merges.

### 2. Submit Before It's Perfect

She submitted the models and API branches before the feature was complete. This got her early feedback and let reviewers start while she continued building.

### 3. Let Stack Handle Rebasing

When she modified `feature/webhook-models`, Stack automatically restacked `feature/webhook-api`. She never ran `git rebase` manually.

### 4. Land As You Go

Instead of waiting for the entire stack to be approved, Alice landed approved PRs throughout the day. This kept her stack small and her changes moving.

### 5. Use the Right Tool for the Job

- **CLI** for quick operations (`gt sync`, `gt create`, `gt land`)
- **TUI** for visual overview and browsing status
- **Draft PRs** for work-in-progress branches

---

## Adapting This to Your Day

Not everyone works like Alice. Here are variations:

### The Review-Heavy Day

If you spend most of your day reviewing others' code:

```bash
# Check what your team has open
gt log --all

# Or in TUI, browse all stacks with h/l
```

### The Hotfix Day

Emergency fix needed:

```bash
gt checkout main
gt create fix/critical-bug
# Fix, test, submit
gt submit
gt land fix/critical-bug
```

No stacking needed — one branch, one PR, land immediately.

### The Refactoring Day

Large refactor across many files:

```bash
# Even big refactors decompose into steps
gt create refactor/extract-service
gt create refactor/extract-service/tests
gt create refactor/extract-service/migrate-callsites
```

Each step is reviewable independently, even if the total change is large.
