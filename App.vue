<style>
canvas {
  width: 100%;
}
</style>
<template>
  <v-app>
    <v-app-bar app flat>
      <v-toolbar-title> Bayesian Binomial Test Calculator</v-toolbar-title>
      <v-spacer> </v-spacer>
      <v-toolbar-items>
        <v-btn text class="text-capitalize" href="https://github.com/tohtsky/bayesian-binomial-test">
          <v-icon> mdi-github</v-icon>
          &nbsp; Source code
        </v-btn>
      </v-toolbar-items>
    </v-app-bar>
    <v-main>
      <v-container>
        <v-row>
          <v-col cols="12" lg="4">
            <v-container>
              <v-row>
                <v-col cols="12">
                  <Group v-model="a" groupName="A" :posInitial="30" :totInitial="100"></Group>
                </v-col>
              </v-row>
              <v-row>
                <v-col cols="12">
                  <Group v-model="b" groupName="B" :posInitial="b_init.pos" :totInitial="b_init.tot"></Group>
                </v-col>
              </v-row>
              <v-row>
                <v-col cols="12">
                  <v-card outlined>
                    <v-card-title> Detailed settings </v-card-title>
                    <v-card-text>
                      <v-row>
                        <v-col cols="6">
                          <v-text-field label="alpha" type="Number" :error-messages="alphaError" v-model.number="alpha">
                          </v-text-field>
                        </v-col>
                        <v-col cols="6">
                          <v-text-field label="beta" type="Number" v-model.number="beta" :error-messages="betaError">
                          </v-text-field>
                        </v-col>
                        <v-col cols="6">
                          <v-text-field label="Number of samples" type="Number" v-model.number="nSamples"
                            :error-messages="nSamplesError"></v-text-field>
                        </v-col>
                        <v-col cols="6">
                          <v-text-field label="Random seed" type="Number" v-model.number="randomSeed"
                            :error-messages="randomSeedError"></v-text-field>
                        </v-col>


                      </v-row>
                    </v-card-text>
                  </v-card>
                </v-col>
              </v-row>
            </v-container>
          </v-col>
          <v-col cols="12" lg="8" :style="{ opacity: payload === null ? 0.4 : 1.0 }">
            <v-row>
              <v-col cols="12">
                <div class="text-center pt-4 text-h5">
                  Probability of B being the winner:
                  {{ (100 * this.result).toFixed(2) }} %
                </div>
                <v-row class="pt-4">
                  <v-col cols="1"></v-col>
                  <v-col cols="10">
                    <canvas ref="canvas_a_b" :width="800 * dpr" :height="300 * dpr"></canvas>
                  </v-col>
                  <v-col cols="1"></v-col>
                  <v-col cols="1"></v-col>
                  <v-col cols="10">
                    <canvas ref="canvas_diff" :width="800 * dpr" :height="300 * dpr"></canvas>
                  </v-col>
                  <v-col cols="1"></v-col>
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



export default {
  data() {
    return {
      a: null,
      b: null,
      a_init: {
        tot: 100,
        pos: 30,
      },
      b_init: {
        tot: 10,
        pos: 5,
      },
      alpha: 1.0,
      alphaError: [],
      beta: 1.0,
      betaError: [],
      nSamples: 100000,
      nSamplesError: [],
      randomSeed: 0,
      randomSeedError: [],
      n_bins: 100,
      result: null,
      dpr: 3,
    };
  },
  computed: {
    payload() {
      let invalid = false;
      if (this.a === null) invalid = true;
      if (this.b === null) invalid = true;
      let alpha = parseFloat(this.alpha);
      let beta = parseFloat(this.beta);
      let nSamples = parseFloat(this.nSamples, 10);
      let randomSeed = parseFloat(this.randomSeed, 10);
      if (isNaN(alpha) || alpha <= 0) {
        this.alphaError = ["valid, positive floating number required."];
        invalid = true;
      } else {
        this.alphaError = null;
      }
      if (isNaN(beta) || beta <= 0) {
        this.betaError = ["valid, positive floating number required."];
        invalid = true;
      } else {
        this.betaError = [];
      }
      if (isNaN(nSamples) || nSamples <= 0 || !Number.isInteger(nSamples)) {
        this.nSamplesError = ["This field must be a strictly positive integer."];
        invalid = true;
      } else {
        this.nSamplesError = [];
      }


      if (isNaN(randomSeed) || randomSeed < 0 || !Number.isInteger(randomSeed)) {
        this.randomSeedError = ["This field must be a non-negative integer."];
        invalid = true;
      } else {
        this.randomSeedError = [];
      }


      if (invalid) {
        return null;
      }

      let data = {
        prior_pos: alpha,
        prior_neg: beta,
        a_tot: this.a.tot,
        a_pos: this.a.pos,
        b_tot: this.b.tot,
        b_pos: this.b.pos,
        n_samples: nSamples,
        n_bins: this.n_bins,
        random_seed: randomSeed,
      };
      return JSON.stringify(data);
    },
  },
  watch: {
    payload(nv) {
      this.compute(nv);
    },
  },
  methods: {
    async compute(payload) {
      console.log(payload);
      if (payload === null) {
        return;
      }

      let res = compute(
        payload,
        this.dpr,
        this.$refs.canvas_a_b,
        this.$refs.canvas_diff
      );
      this.result = res;
    },
  },
  async mounted() {
    //this.compute(this.payload);
  },
  components: {
    Group,
  },
};
</script>
