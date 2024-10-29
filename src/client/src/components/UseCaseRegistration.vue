<template>
    <div>
      <h2>Use Case Registration</h2>
      <form @submit.prevent='handleSubmit'>
        <label for='use-case-name'>Use Case Name:</label>
        <input type='text' id='use-case-name' v-model='useCaseName' required />
        <br />
        <label for='use-case-description'>Description:</label>
        <textarea id='use-case-description' v-model='useCaseDescription' required></textarea>
        <br />
        <button type='submit'>Register Use Case</button>
      </form>
      <p v-if='submitted'>Use Case registered: {{ useCaseName }}</p>
    </div>
  </template>
  
  <script>
  export default {
    data() {
      return {
        useCaseName: '',
        useCaseDescription: '',
        submitted: false
      };
    },
    methods: {
      handleSubmit() {
        this.submitted = true;
        // Logic to handle registration can be added here.
      }
    }
  };
  </script>