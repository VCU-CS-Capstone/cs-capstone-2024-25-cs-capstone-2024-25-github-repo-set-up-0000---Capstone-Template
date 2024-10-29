<template>
  <div id="app">
    <nav>
      <router-link to="/">Home</router-link> |
      <router-link to="/usecase">Use Case Registration</router-link> |
      <router-link to="/dataset">Dataset Registration</router-link> |
    </nav>
    <router-view></router-view>
  </div>
</template>

<script>
export default {
  name: 'App'
}
</script>