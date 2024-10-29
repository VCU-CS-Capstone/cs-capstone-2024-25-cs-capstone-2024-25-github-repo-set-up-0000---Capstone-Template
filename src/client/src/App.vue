<template>
  <div id="app">
    <header class="header">
      <h1>Gallery</h1>
      <nav>
        <router-link to="/">Dashboard</router-link>
        <router-link to="/usecase">Use cases</router-link>
        <router-link to="/dataset">Datasets</router-link>
      </nav>
    </header>

    <router-view></router-view>
  </div>
</template>

<script>
export default {
  name: 'App'
}
</script>

<style scoped>
#app {
  font-family: Arial, sans-serif;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  background-color: #FFFFFF;
  color: #004d66;
  padding: 10px;
}

.header nav a {
  color: #004d66;
  margin-right: 15px;
}
</style>