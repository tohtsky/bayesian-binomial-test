<template>
  <v-card variant="outlined">
    <v-card-title>Group {{ groupName }}</v-card-title>
    <v-card-text>
      <v-row>
        <v-col cols="5">
          <v-text-field
            v-model="total"
            label="Total trials"
            type="number"
            :error-messages="totalErrors"
          />
        </v-col>
        <v-col cols="1"></v-col>
        <v-col cols="5">
          <v-text-field
            v-model="successes"
            label="Number of successes"
            type="number"
            :error-messages="successErrors"
          />
        </v-col>
        <v-col cols="1"></v-col>
      </v-row>
    </v-card-text>
  </v-card>
</template>

<script>
function asNonNegativeInteger(value) {
  if (value === "" || value === null || value === undefined) return null;
  const number = Number(value);
  return Number.isInteger(number) && number >= 0 ? number : null;
}

export default {
  props: {
    modelValue: {
      type: Object,
      required: true,
    },
    groupName: {
      type: String,
      required: true,
    },
  },
  emits: ["update:modelValue"],
  computed: {
    total: {
      get() {
        return this.modelValue.tot;
      },
      set(tot) {
        this.$emit("update:modelValue", { ...this.modelValue, tot });
      },
    },
    successes: {
      get() {
        return this.modelValue.pos;
      },
      set(pos) {
        this.$emit("update:modelValue", { ...this.modelValue, pos });
      },
    },
    totalErrors() {
      const total = asNonNegativeInteger(this.modelValue.tot);
      const successes = asNonNegativeInteger(this.modelValue.pos);
      if (total === null) {
        return ["A non-negative integer is required."];
      }
      return successes !== null && total < successes
        ? ["Cannot be smaller than the number of successes."]
        : [];
    },
    successErrors() {
      const total = asNonNegativeInteger(this.modelValue.tot);
      const successes = asNonNegativeInteger(this.modelValue.pos);
      if (successes === null) {
        return ["A non-negative integer is required."];
      }
      return total !== null && successes > total
        ? ["Cannot be greater than total trials."]
        : [];
    },
  },
};
</script>
