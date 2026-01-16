# Contributing to `enumflags2`

## LLM policy

- We do not use LLMs.
- We do not work with people who use LLMs for their `enumflags2` contributions.
- We do not welcome people who make the existence of LLMs our problem.

By asking for our labor, you're relying on our goodwill and the desire to
contribute to the commons of open source. Violations of the LLM policy are
generally a particularly good way to lose this goodwill.

Exemptions from this policy are available under enterprise support contracts;
contact us for a quote.

## Other notes

The test suite includes tests that verify the diagnostics output by the proc
macro do not regress. This is somewhat fiddly, because highlighting a span
within a diagnostic is only properly possible on nightly – stable gets a
"polyfill" that only underlines the first token of the span.

As such, the expected output in the `.stderr` files is the one for the nightly
compiler. The `ui` tests will fail if ran on stable.

Moreover, if the project hasn't been touched for a while, rustc's diagnostic
output might've drifted somewhat. If CI fails because of this, you need to
bless the output locally. Give the diff a once over to make sure there aren't
any regressions to diagnostic quality.
