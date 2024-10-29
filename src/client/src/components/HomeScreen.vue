<template>
  <div class="dashboard">
    <div class="content">
      <div class="panel use-cases">
        <h2>Use Cases</h2>
        <ul>
          <li v-for="useCase in useCases" :key="useCase.id">
            {{ useCase.name }}
            <button @click="edit(useCase)">✏️</button>
            <button @click="favorite(useCase)">⭐</button>
          </li>
        </ul>
        <div class="actions">
          <button @click="addNewUseCase">Add New</button>
          <button @click="reviewPendingUseCases">Review</button>
          <button>View All</button>
        </div>
      </div>

      <div class="panel datasets">
        <h2>Datasets</h2>
        <ul>
          <li v-for="dataset in datasets" :key="dataset.id">
            {{ dataset.name }}
            <button @click="edit(dataset)">✏️</button>
            <button @click="favorite(dataset)">⭐</button>
          </li>
        </ul>
        <div class="actions">
          <button @click="addNewDataset">Add New</button>
          <button @click="reviewPendingDatasets">Review</button>
          <button>View All</button>
        </div>
      </div>

      <div class="main-actions">
        <h2>What would you like to do today?</h2>
        <p>We'll guide you through filling in this info. You can always save, step away and return right where you left off.</p>
        <button @click="addNewUseCase">Add new use case</button>
        <button @click="reviewPendingUseCases">Review pending use cases</button>
        <button @click="addNewDataset">Add new dataset</button>
        <button @click="reviewPendingDatasets">Review pending datasets</button>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  data() {
    return {
      useCases: [
        { id: 1, name: 'Use case name 1' },
        { id: 2, name: 'Use case name 2' },
        { id: 3, name: 'Use case name 3' },
      ],
      datasets: [
        { id: 1, name: 'Dataset name 1' },
        { id: 2, name: 'Dataset name 2' },
        { id: 3, name: 'Dataset name 3' },
      ],
    };
  },
  methods: {
    addNewUseCase() {
      // Logic to add a new use case
    },
    reviewPendingUseCases() {
      // Logic to review pending use cases
    },
    addNewDataset() {
      // Logic to add a new dataset
    },
    reviewPendingDatasets() {
      // Logic to review pending datasets
    },
  },
};
</script>

<style scoped>
.dashboard {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  background-color: #004d66;
  padding: 20px;
}

.content {
  display: flex;
}

.panel {
  background-color: white;
  padding: 20px;
  border-radius: 8px;
  margin-right: 20px;
}

.panel ul {
  list-style-type: none;
}

.panel .actions {
  margin-top: 10px;
}

.main-actions {
  max-width: 300px;
}

.main-actions h2,
.main-actions p {
  color: white;
}
</style>