<template>

  <div>

    <h1>运维控制台</h1>

    <button @click="start">启动 nginx</button>

    <p>{{ msg }}</p>

  </div>

</template>

<script setup>

import { ref } from "vue";

import { invoke } from "@tauri-apps/api/core";

const msg = ref("");

const start = async () => {

  msg.value = await invoke("start_nginx");

};

</script>
