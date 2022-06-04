# Bayesian binomial test using WebAssembly

[**Demo**](https://tohtsky.github.io/bayesian-binomial-test/)


A/B test significance judgement using bayesian beta-binomial distribution is simple. In Python, we can do that like:
```python
import numpy as np

alpha_prior = 1.0
beta_prior = 1.0

n_events_a = 100
n_success_a = 30

n_events_b = 10
n_success_b = 5

n_samples = 10000

rng = np.random.default_rng(0)

posterior_a = rng.beta(alpha_prior + n_success_a, beta_prior + n_events_a - n_success_a, size=n_samples)
posterior_b = rng.beta(alpha_prior + n_success_b, beta_prior + n_events_b - n_success_b, size=n_samples)

# 0.90576
(posterior_a < posterior_b).mean()
```

This simple demo visualizes the Bayesian binomial test, using Rust + WebAssembly to sample from the posterior distribution (obviously overkill, but fun).

This should work off-line. Get a single HTML file from [the latest release](https://github.com/tohtsky/bayesian-binomial-test/releases/latest).

A similar application that influcend this project can be found [here](https://making.lyst.com/bayesian-calculator/).