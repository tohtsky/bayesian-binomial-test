export const DEFAULT_PARAMETERS = Object.freeze({
  aTotal: 100,
  aSuccesses: 30,
  bTotal: 10,
  bSuccesses: 5,
  alpha: 1,
  beta: 1,
  samples: 100000,
  seed: 0,
});

const QUERY_KEYS = Object.freeze({
  aTotal: "a_total",
  aSuccesses: "a_successes",
  bTotal: "b_total",
  bSuccesses: "b_successes",
  alpha: "alpha",
  beta: "beta",
  samples: "samples",
  seed: "seed",
});

function readNumber(searchParams, key, fallback, isValid) {
  const raw = searchParams.get(key);
  if (raw === null || raw.trim() === "") return fallback;
  const value = Number(raw);
  return Number.isFinite(value) && isValid(value) ? value : fallback;
}

export function readParameters(search, defaults = DEFAULT_PARAMETERS) {
  const query = new URLSearchParams(search);
  const parameters = {
    aTotal: readNumber(
      query,
      QUERY_KEYS.aTotal,
      defaults.aTotal,
      (value) => Number.isInteger(value) && value >= 0
    ),
    aSuccesses: readNumber(
      query,
      QUERY_KEYS.aSuccesses,
      defaults.aSuccesses,
      (value) => Number.isInteger(value) && value >= 0
    ),
    bTotal: readNumber(
      query,
      QUERY_KEYS.bTotal,
      defaults.bTotal,
      (value) => Number.isInteger(value) && value >= 0
    ),
    bSuccesses: readNumber(
      query,
      QUERY_KEYS.bSuccesses,
      defaults.bSuccesses,
      (value) => Number.isInteger(value) && value >= 0
    ),
    alpha: readNumber(
      query,
      QUERY_KEYS.alpha,
      defaults.alpha,
      (value) => value > 0
    ),
    beta: readNumber(
      query,
      QUERY_KEYS.beta,
      defaults.beta,
      (value) => value > 0
    ),
    samples: readNumber(
      query,
      QUERY_KEYS.samples,
      defaults.samples,
      (value) => Number.isInteger(value) && value > 0
    ),
    seed: readNumber(
      query,
      QUERY_KEYS.seed,
      defaults.seed,
      (value) => Number.isInteger(value) && value >= 0
    ),
  };

  if (parameters.aSuccesses > parameters.aTotal) {
    parameters.aTotal = defaults.aTotal;
    parameters.aSuccesses = defaults.aSuccesses;
  }
  if (parameters.bSuccesses > parameters.bTotal) {
    parameters.bTotal = defaults.bTotal;
    parameters.bSuccesses = defaults.bSuccesses;
  }

  return parameters;
}

export function parametersToUrl(href, parameters) {
  const url = new URL(href);
  for (const [parameter, queryKey] of Object.entries(QUERY_KEYS)) {
    url.searchParams.set(queryKey, String(parameters[parameter]));
  }
  return url;
}
