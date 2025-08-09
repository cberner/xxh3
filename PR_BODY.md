Refactor: replace accumulate_block() and large helpers with macros

Summary
- Replace the accumulate_block() function in src/xxh3.rs with a macro (accumulate_block!) that inlines the stripe-accumulation loop.
- Also replace hash_large_helper(), hash64_large_generic(), and hash128_large_generic() with macros:
  - hash_large_helper!
  - hash64_large_generic!
  - hash128_large_generic!
- Move all macro definitions near the top of the file so they are defined before use.
- Update all call sites (generic and arch-specific wrappers) to use the new macros.
- Goal: reduce unsafe function pointer indirection and set up for future refactors toward safer intrinsics under target_feature contexts.

Rationale
- The previous functions were thin wrappers that relied on unsafe function pointers (accum_stripe, scramble, gen_secret).
- Converting to macros inlines the logic at call sites, avoiding function-pointer indirection and keeping usage within the correct target_feature-enabled contexts.

Testing
- Ran locally:
  - just pre (fmt, clippy, deny) — passed
  - just test — passed
  - just fuzz_ci — passed locally (1 minute run)
- No behavior changes are expected; macros preserve the previous logic and arguments.

Links
- Link to Devin run: https://app.devin.ai/sessions/bcc4a0bb7f8e4dc2afb288d9f10f9b7d
- Requested by: Christopher Berner (@cberner)
