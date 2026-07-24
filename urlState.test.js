import test from "node:test";
import assert from "node:assert/strict";
import {
  DEFAULT_PARAMETERS,
  parametersToUrl,
  readParameters,
} from "./urlState.js";

test("reads every supported parameter from the query string", () => {
  const result = readParameters(
    "?a_total=20&a_successes=4&b_total=30&b_successes=9" +
      "&alpha=0.5&beta=2.5&samples=1234&seed=42"
  );

  assert.deepEqual(result, {
    aTotal: 20,
    aSuccesses: 4,
    bTotal: 30,
    bSuccesses: 9,
    alpha: 0.5,
    beta: 2.5,
    samples: 1234,
    seed: 42,
  });
});

test("falls back for invalid values and inconsistent groups", () => {
  const result = readParameters(
    "?a_total=2&a_successes=3&b_total=-1&alpha=0&samples=1.5&seed=-2"
  );

  assert.deepEqual(result, DEFAULT_PARAMETERS);
});

test("writes parameters while preserving unrelated query parameters and hash", () => {
  const url = parametersToUrl(
    "https://example.com/calculator?utm_source=test#results",
    {
      ...DEFAULT_PARAMETERS,
      aTotal: 200,
      aSuccesses: 80,
    }
  );

  assert.equal(url.searchParams.get("utm_source"), "test");
  assert.equal(url.searchParams.get("a_total"), "200");
  assert.equal(url.searchParams.get("a_successes"), "80");
  assert.equal(url.hash, "#results");
});
