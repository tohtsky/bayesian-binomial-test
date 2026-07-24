<template>
  <v-app>
    <v-app-bar flat>
      <v-toolbar-title>Bayesian Binomial Test Calculator</v-toolbar-title>
      <v-spacer />
      <v-btn
        variant="text"
        class="text-none"
        href="https://github.com/tohtsky/bayesian-binomial-test"
      >
        <v-icon icon="mdi-github" />
        &nbsp; Source code
      </v-btn>
    </v-app-bar>

    <v-main>
      <v-container>
        <v-row>
          <v-col cols="12" lg="4">
            <v-container>
              <v-row>
                <v-col cols="12">
                  <Group v-model="a" group-name="A" />
                </v-col>
              </v-row>
              <v-row>
                <v-col cols="12">
                  <Group v-model="b" group-name="B" />
                </v-col>
              </v-row>
              <v-row>
                <v-col cols="12">
                  <v-card variant="outlined">
                    <v-card-title>Detailed settings</v-card-title>
                    <v-card-text>
                      <v-row>
                        <v-col cols="6">
                          <v-text-field
                            v-model="alpha"
                            label="alpha"
                            type="number"
                            :error-messages="alphaErrors"
                          />
                        </v-col>
                        <v-col cols="6">
                          <v-text-field
                            v-model="beta"
                            label="beta"
                            type="number"
                            :error-messages="betaErrors"
                          />
                        </v-col>
                        <v-col cols="6">
                          <v-text-field
                            v-model="nSamples"
                            label="Number of samples"
                            type="number"
                            :error-messages="nSamplesErrors"
                          />
                        </v-col>
                        <v-col cols="6">
                          <v-text-field
                            v-model="randomSeed"
                            label="Random seed"
                            type="number"
                            :error-messages="randomSeedErrors"
                          />
                        </v-col>
                      </v-row>
                    </v-card-text>
                  </v-card>
                </v-col>
              </v-row>
            </v-container>
          </v-col>

          <v-col cols="12" lg="8" :style="{ opacity: payload === null ? 0.4 : 1 }">
            <v-row>
              <v-col cols="12">
                <div class="text-center pt-4 text-h5">
                  Probability of B being the winner:
                  {{ formattedResult }}
                </div>
                <div v-if="computeError" class="text-center text-error pt-2">
                  {{ computeError }}
                </div>
                <v-row class="pt-4">
                  <v-col cols="1" />
                  <v-col cols="10">
                    <canvas
                      ref="canvas_a_b"
                      :width="800 * dpr"
                      :height="300 * dpr"
                    />
                  </v-col>
                  <v-col cols="1" />
                  <v-col cols="1" />
                  <v-col cols="10">
                    <canvas
                      ref="canvas_diff"
                      :width="800 * dpr"
                      :height="300 * dpr"
                    />
                  </v-col>
                  <v-col cols="1" />
                </v-row>
              </v-col>
            </v-row>
          </v-col>
        </v-row>
      </v-container>
    </v-main>
  </v-app>
</template>

<script>
import { compute } from "./bayesian-wasm/pkg";
import Group from "./components/Group.vue";
import {
  DEFAULT_PARAMETERS,
  parametersToUrl,
  readParameters,
} from "./urlState.js";

function positiveNumber(value) {
  if (value === "" || value === null || value === undefined) return null;
  const number = Number(value);
  return Number.isFinite(number) && number > 0 ? number : null;
}

function nonNegativeInteger(value) {
  if (value === "" || value === null || value === undefined) return null;
  const number = Number(value);
  return Number.isInteger(number) && number >= 0 ? number : null;
}

function validGroup(group) {
  const tot = nonNegativeInteger(group.tot);
  const pos = nonNegativeInteger(group.pos);
  return tot !== null && pos !== null && pos <= tot ? { tot, pos } : null;
}

export default {
  components: {
    Group,
  },
  data() {
    const initial = readParameters(window.location.search);
    return {
      a: { tot: initial.aTotal, pos: initial.aSuccesses },
      b: { tot: initial.bTotal, pos: initial.bSuccesses },
      alpha: initial.alpha,
      beta: initial.beta,
      nSamples: initial.samples,
      randomSeed: initial.seed,
      nBins: 100,
      result: null,
      computeError: "",
      dpr: Math.min(3, Math.max(1, window.devicePixelRatio || 1)),
      mountedReady: false,
    };
  },
  computed: {
    alphaErrors() {
      return positiveNumber(this.alpha) === null
        ? ["A positive number is required."]
        : [];
    },
    betaErrors() {
      return positiveNumber(this.beta) === null
        ? ["A positive number is required."]
        : [];
    },
    nSamplesErrors() {
      const value = nonNegativeInteger(this.nSamples);
      return value === null || value === 0
        ? ["A strictly positive integer is required."]
        : [];
    },
    randomSeedErrors() {
      return nonNegativeInteger(this.randomSeed) === null
        ? ["A non-negative integer is required."]
        : [];
    },
    parameters() {
      const a = validGroup(this.a);
      const b = validGroup(this.b);
      const alpha = positiveNumber(this.alpha);
      const beta = positiveNumber(this.beta);
      const samples = nonNegativeInteger(this.nSamples);
      const seed = nonNegativeInteger(this.randomSeed);

      if (
        a === null ||
        b === null ||
        alpha === null ||
        beta === null ||
        samples === null ||
        samples === 0 ||
        seed === null
      ) {
        return null;
      }

      return {
        aTotal: a.tot,
        aSuccesses: a.pos,
        bTotal: b.tot,
        bSuccesses: b.pos,
        alpha,
        beta,
        samples,
        seed,
      };
    },
    payload() {
      if (this.parameters === null) return null;
      return JSON.stringify({
        prior_pos: this.parameters.alpha,
        prior_neg: this.parameters.beta,
        a_tot: this.parameters.aTotal,
        a_pos: this.parameters.aSuccesses,
        b_tot: this.parameters.bTotal,
        b_pos: this.parameters.bSuccesses,
        n_samples: this.parameters.samples,
        n_bins: this.nBins,
        random_seed: this.parameters.seed,
      });
    },
    formattedResult() {
      return this.result === null ? "—" : `${(100 * this.result).toFixed(2)} %`;
    },
  },
  watch: {
    payload(payload) {
      if (!this.mountedReady || payload === null) return;
      this.updateUrl();
      this.computeResult(payload);
    },
  },
  mounted() {
    this.mountedReady = true;
    this.updateUrl();
    this.computeResult(this.payload);
    window.addEventListener("popstate", this.restoreFromUrl);
  },
  beforeUnmount() {
    window.removeEventListener("popstate", this.restoreFromUrl);
  },
  methods: {
    updateUrl() {
      if (this.parameters === null) return;
      const url = parametersToUrl(window.location.href, this.parameters);
      window.history.replaceState(window.history.state, "", url);
    },
    restoreFromUrl() {
      const parameters = readParameters(
        window.location.search,
        DEFAULT_PARAMETERS
      );
      this.a = { tot: parameters.aTotal, pos: parameters.aSuccesses };
      this.b = { tot: parameters.bTotal, pos: parameters.bSuccesses };
      this.alpha = parameters.alpha;
      this.beta = parameters.beta;
      this.nSamples = parameters.samples;
      this.randomSeed = parameters.seed;
    },
    computeResult(payload) {
      if (payload === null) {
        this.result = null;
        return;
      }

      try {
        this.computeError = "";
        this.result = compute(
          payload,
          this.dpr,
          this.$refs.canvas_a_b,
          this.$refs.canvas_diff
        );
      } catch (error) {
        this.result = null;
        this.computeError = `Calculation failed: ${String(error)}`;
      }
    },
  },
};
</script>

<style>
canvas {
  display: block;
  width: 100%;
}
</style>
