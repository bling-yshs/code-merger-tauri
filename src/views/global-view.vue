<template>
  <div class="p-20">
    <router-view />
    <route-toggle />
    <dark-toggle />
  </div>
</template>
<script setup lang="ts">
import DarkToggle from '@/compoments/theme-toggle.vue'
import RouteToggle from '@/compoments/route-toggle.vue'
</script>
