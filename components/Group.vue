<template>
  <v-card outlined>
    <v-card-title> Group {{ groupName }}</v-card-title>
    <v-card-text>
      <v-row>
        <v-col cols="5">
          <v-text-field label="Total trials" type="Number" v-model.number="tot" :error-messages="totError">
          </v-text-field>
        </v-col>
        <v-col cols="1"></v-col>
        <v-col cols="5">
          <v-text-field label="Number of successes" type="Number" v-model.number="pos" :error-messages="posError">
          </v-text-field>
        </v-col>
        <v-col cols="1"></v-col>
      </v-row>
    </v-card-text>
  </v-card>
</template>

<script>
export default {
  props: {
    value: {
      type: Object,
      default: {},
    },
    totInitial: {
      type: Number,
      required: true,
    },
    posInitial: {
      type: Number,
      required: true,
    },
    groupName: {
      type: String,
      required: true,
    },
  },
  data() {
    return {
      tot: this.totInitial,
      pos: this.posInitial,
      totError: [],
      posError: [],
    };
  },
  computed: {
    payload() {
      let pos = parseInt(this.pos);
      let tot = parseInt(this.tot);
      let invalid = false;

      let totError = [];
      let posError = [];
      if (isNaN(tot)) {
        totError.push("The field must be a valid integer");
        invalid = true;
      }
      if (tot < 0) {
        totError.push("Must be non-negative.");
        invalid = true;
      }
      if (isNaN(pos)) {
        posError.push("The field required");
        invalid = true;
      }
      if (pos < 0) {
        posError.push("Must be non-negative.");
        invalid = true;
      }
      if (!invalid) {
        if (tot < pos) {
          totError.push("Cannot be smaller than the number of successes");
          posError.push("Cannot be greater than total trials.");
          invalid = true;
        }
      }
      this.totError = totError;
      this.posError = posError;
      if (invalid) return null;
      return JSON.stringify({
        pos,
        tot,
      });
    },
  },
  watch: {
    payload(nv) {
      if (nv === null) {
        this.$emit("input", null);
      }
      this.$emit("input", JSON.parse(nv));
    },
  },
  mounted() {
    this.$emit("input", JSON.parse(this.payload));
  },
};
</script>
