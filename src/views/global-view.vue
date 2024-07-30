<template>
  <div class="p-20">
    <router-view />
    <dark-toggle />
  </div>
</template>
<script setup lang="ts">
import DarkToggle from '@/compoments/dark-toggle.vue'
</script>
