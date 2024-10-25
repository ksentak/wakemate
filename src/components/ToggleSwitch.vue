<template>
  <div
    class="min-h-screen flex flex-col items-center bg-gradient-to-b from-stone-900 to-stone-800"
  >
    <h1
      class="text-5xl font-bold font-mono bg-gradient-to-r from-violet-700 via-red-500 to-yellow-500 bg-clip-text text-transparent mt-12 mb-12"
    >
      WakeMate
    </h1>
    <button
      @click="toggle"
      class="flex items-center justify-center transition-transform transform hover:scale-110 duration-300 ease-in-out relative overflow-hidden"
    >
      <font-awesome-icon
        :icon="faPowerOff"
        :class="[
          'relative duration-300 ease-in-out text-8xl',
          isEnabled ? 'text-yellow-600' : 'text-zinc-700',
        ]"
      />
    </button>
  </div>
</template>

<script setup>
import { ref } from 'vue';
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome';
import { faPowerOff } from '@fortawesome/free-solid-svg-icons';

const isEnabled = ref(false);

const toggle = () => {
  isEnabled.value = !isEnabled.value;
};
</script>
